// A small Chrome DevTools Protocol driver: Node's built-in WebSocket and fetch, nothing else.
//
// Selectors throughout are JavaScript *expressions* that evaluate to an element (or an array of
// them), written as strings, so a check can say `byText('button', 'Options')` as easily as
// `q('[role=tab]')`. Build them with the helpers at the bottom.

import { writeFileSync } from 'node:fs';

export const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

export class Skip extends Error {}

const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);

export class Browser {
  constructor(port) {
    this.port = port;
    this.base = `http://127.0.0.1:${port}`;
  }

  static async connect(port, timeoutMs = 20000) {
    const b = new Browser(port);
    const end = Date.now() + timeoutMs;
    for (;;) {
      try {
        const r = await fetch(`${b.base}/json/version`);
        if (r.ok) return b;
      } catch {}
      if (Date.now() > end) throw new Error(`chromium did not open its debugging port ${port}`);
      await sleep(150);
    }
  }

  async page(origin, { width = 1280, height = 900 } = {}) {
    const info = await (await fetch(`${this.base}/json/new?about:blank`, { method: 'PUT' })).json();
    const p = new Page(this, info, origin);
    await p.init(width, height);
    return p;
  }
}

export class Page {
  constructor(browser, info, origin) {
    this.browser = browser;
    this.info = info;
    this.origin = origin;
    this.failures = [];
    this.consoleProblems = [];
    this.label = 'page';
    this.id = 0;
    this.pending = new Map();
  }

  async init(width, height) {
    this.ws = new WebSocket(this.info.webSocketDebuggerUrl);
    await new Promise((res, rej) => {
      this.ws.addEventListener('open', res, { once: true });
      this.ws.addEventListener('error', () => rej(new Error('websocket error')), { once: true });
    });
    this.ws.addEventListener('message', (m) => this.#message(JSON.parse(m.data)));
    await this.send('Page.enable');
    await this.send('Runtime.enable');
    await this.viewport(width, height);
  }

  #message(d) {
    if (d.id && this.pending.has(d.id)) {
      const { res, rej } = this.pending.get(d.id);
      this.pending.delete(d.id);
      d.error ? rej(new Error(`${d.error.message}`)) : res(d.result);
      return;
    }
    if (d.method === 'Runtime.consoleAPICalled' && ['error', 'warning'].includes(d.params.type)) {
      const text = d.params.args.map((a) => a.value ?? a.description).join(' ');
      this.consoleProblems.push(`console.${d.params.type}: ${text}`);
    }
    if (d.method === 'Runtime.exceptionThrown') {
      const x = d.params.exceptionDetails;
      this.consoleProblems.push(`exception: ${x.exception?.description || x.text}`);
    }
  }

  send(method, params = {}) {
    return new Promise((res, rej) => {
      const i = ++this.id;
      this.pending.set(i, { res, rej });
      this.ws.send(JSON.stringify({ id: i, method, params }));
    });
  }

  sleep = sleep;

  async viewport(width, height, mobile = false) {
    await this.send('Emulation.setDeviceMetricsOverride', { width, height, deviceScaleFactor: 1, mobile });
  }

  async ev(expression) {
    const r = await this.send('Runtime.evaluate', { expression, awaitPromise: true, returnByValue: true });
    if (r.exceptionDetails) {
      throw new Error(r.exceptionDetails.exception?.description || r.exceptionDetails.text);
    }
    return r.result.value;
  }

  async goto(path, { settle = 400 } = {}) {
    const url = path.startsWith('http') ? path : this.origin + path;
    await this.send('Page.navigate', { url });
    const end = Date.now() + 20000;
    for (;;) {
      if (await this.ev(`!!document.querySelector('[data-slot]')`).catch(() => false)) break;
      if (Date.now() > end) throw new Error(`no [data-slot] element after loading ${path}`);
      await sleep(100);
    }
    await sleep(settle);
  }

  async waitFor(expr, ms = 3000) {
    const end = Date.now() + ms;
    for (;;) {
      if (await this.ev(`!!(${expr})`).catch(() => false)) return true;
      if (Date.now() > end) return false;
      await sleep(80);
    }
  }

  async exists(sel) {
    return this.ev(`!!(${sel})`);
  }

  async center(sel) {
    if (!(await this.waitFor(sel))) throw new Error(`no element: ${sel}`);
    const r = await this.ev(`(() => {
      const e = ${sel};
      e.scrollIntoView({ block: 'center', inline: 'center' });
      const b = e.getBoundingClientRect();
      return [b.x + b.width / 2, b.y + b.height / 2];
    })()`);
    return r;
  }

  async mouse(type, x, y, button = 'left') {
    await this.send('Input.dispatchMouseEvent', {
      type,
      x,
      y,
      button: type === 'mouseMoved' ? 'none' : button,
      clickCount: type === 'mouseMoved' ? 0 : 1,
    });
  }

  async hover(sel, wait = 250) {
    const [x, y] = await this.center(sel);
    await this.mouse('mouseMoved', x, y);
    await sleep(wait);
  }

  async click(sel, wait = 250) {
    const [x, y] = await this.center(sel);
    await this.mouse('mouseMoved', x, y);
    await this.mouse('mousePressed', x, y);
    await this.mouse('mouseReleased', x, y);
    await sleep(wait);
  }

  async clickAt(x, y, wait = 250) {
    await this.mouse('mouseMoved', x, y);
    await this.mouse('mousePressed', x, y);
    await this.mouse('mouseReleased', x, y);
    await sleep(wait);
  }

  async focus(sel) {
    if (!(await this.waitFor(sel))) throw new Error(`no element: ${sel}`);
    await this.ev(`(${sel}).focus()`);
    await sleep(80);
  }

  async key(k, wait = 200) {
    const codes = { Enter: 13, Escape: 27, ArrowDown: 40, ArrowUp: 38, ArrowRight: 39, ArrowLeft: 37, ' ': 32, Tab: 9, Home: 36, End: 35, Backspace: 8 };
    const code = codes[k] ?? k.toUpperCase().charCodeAt(0);
    const base = { key: k, code: k === ' ' ? 'Space' : k, windowsVirtualKeyCode: code };
    await this.send('Input.dispatchKeyEvent', { type: 'keyDown', ...base, text: k === 'Enter' ? '\r' : k.length === 1 ? k : undefined });
    await this.send('Input.dispatchKeyEvent', { type: 'keyUp', ...base });
    await sleep(wait);
  }

  async type(text, wait = 250) {
    await this.send('Input.insertText', { text });
    await sleep(wait);
  }

  async shot(path) {
    const r = await this.send('Page.captureScreenshot', { format: 'png' });
    writeFileSync(path, Buffer.from(r.data, 'base64'));
  }

  attr(sel, name) {
    return this.ev(`(${sel})?.getAttribute(${JSON.stringify(name)}) ?? null`);
  }

  text(sel) {
    return this.ev(`(${sel})?.textContent.trim() ?? null`);
  }

  fail(label, detail) {
    this.failures.push({ page: this.label, label, detail });
  }

  expect(label, actual, expected) {
    if (!same(actual, expected)) {
      this.fail(label, `expected ${JSON.stringify(expected)}, got ${JSON.stringify(actual)}`);
      return false;
    }
    return true;
  }

  ok(label, cond) {
    if (!cond) this.fail(label, 'expected a truthy value');
    return !!cond;
  }

  // Polls an expression until it equals `expected`, then records the verdict. Prefer this to a
  // fixed sleep after anything animated or debounced.
  async settle(label, expr, expected, ms = 2500) {
    const end = Date.now() + ms;
    let got;
    for (;;) {
      got = await this.ev(expr).catch((e) => `error: ${e.message}`);
      if (same(got, expected)) return true;
      if (Date.now() > end) break;
      await sleep(80);
    }
    this.fail(label, `expected ${JSON.stringify(expected)}, got ${JSON.stringify(got)}`);
    return false;
  }

  skip(reason) {
    throw new Skip(reason);
  }

  async close() {
    try {
      this.ws.close();
    } catch {}
    await fetch(`${this.browser.base}/json/close/${this.info.id}`).catch(() => {});
  }
}

// ---- selector expression builders -------------------------------------------------------------

const J = JSON.stringify;

export const q = (css, root = 'document') => `${root}.querySelector(${J(css)})`;
export const qa = (css, root = 'document') => `[...${root}.querySelectorAll(${J(css)})]`;

// The site header lives inside <main> and carries a theme-switcher popover and a sidebar trigger,
// so a bare `main button` or `main [data-slot=popover-trigger]` finds those first. Everything that
// searches the demo goes through `demo`.
export const demo = (css) =>
  `[...document.querySelectorAll('main ${css}')].filter((e) => !e.closest('header'))`;
export const demo1 = (css) => `${demo(css)}[0]`;

export const byText = (css, text, { exact = true, root = 'document' } = {}) =>
  `[...${root}.querySelectorAll(${J(css)})].find((e) => !e.closest('header') && ${
    exact ? `e.textContent.trim() === ${J(text)}` : `e.textContent.trim().startsWith(${J(text)})`
  })`;

export const demoBtn = (text) =>
  `${demo('button')}.find((e) => e.textContent.trim() === ${J(text)})`;

export const menuItem = (text) =>
  `[...document.querySelectorAll('[role^=menuitem]')].find((e) => e.textContent.trim().startsWith(${J(text)}))`;

// Everything inside the DemoSection whose title matches `re`.
export const section = (re) =>
  `([...document.querySelectorAll('[data-toc]')].find((h) => ${re}.test(h.textContent))?.closest('section') ?? document.querySelector('main'))`;

export const active = `document.activeElement`;
export const activeText = `document.activeElement.textContent.trim()`;
