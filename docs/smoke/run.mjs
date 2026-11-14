#!/usr/bin/env node
// Browser smoke suite for the docs site. No dependencies: Node's WebSocket and fetch, driving a
// headless Chromium over the DevTools protocol.
//
//   node docs/smoke/run.mjs [--url <base>] [--only <substr>] [--no-sweep]

import { spawn } from 'node:child_process';
import { existsSync, mkdtempSync, readdirSync, rmSync } from 'node:fs';
import { createServer } from 'node:net';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

import { Browser, Skip, sleep } from './cdp.mjs';
import { pages, sweep } from './sweep.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const docsDir = join(here, '..');

const args = process.argv.slice(2);
const flag = (n) => args.includes(n);
const value = (n) => (args.indexOf(n) >= 0 ? args[args.indexOf(n) + 1] : undefined);
if (flag('--help') || flag('-h')) {
  console.log('usage: node docs/smoke/run.mjs [--url <base>] [--only <substr>] [--no-sweep]');
  process.exit(0);
}
const only = value('--only');
const wanted = (name) => !only || name.includes(only);

const children = [];
const tmpdirs = [];

const freePort = () =>
  new Promise((res, rej) => {
    const s = createServer();
    s.once('error', rej);
    s.listen(0, '127.0.0.1', () => {
      const { port } = s.address();
      s.close(() => res(port));
    });
  });

const alive = (pid) => {
  try {
    process.kill(pid, 0);
    return true;
  } catch {
    return false;
  }
};

const launch = (cmd, argv, opts) => {
  const c = spawn(cmd, argv, { detached: true, stdio: ['ignore', 'pipe', 'pipe'], ...opts });
  c.out = '';
  const keep = (d) => (c.out = (c.out + d).slice(-8000));
  c.stdout.on('data', keep);
  c.stderr.on('data', keep);
  c.spawnError = null;
  c.on('error', (e) => (c.spawnError = e));
  children.push(c);
  return c;
};

let cleaned = false;
async function cleanup() {
  if (cleaned) return;
  cleaned = true;
  for (const c of children) {
    if (c.pid) {
      try {
        process.kill(-c.pid, 'SIGTERM');
      } catch {}
    }
  }
  const end = Date.now() + 5000;
  while (Date.now() < end && children.some((c) => c.pid && alive(c.pid))) await sleep(100);
  for (const c of children) {
    if (c.pid) {
      try {
        process.kill(-c.pid, 'SIGKILL');
      } catch {}
    }
  }
  for (const d of tmpdirs) rmSync(d, { recursive: true, force: true });
}

for (const sig of ['SIGINT', 'SIGTERM', 'SIGHUP']) {
  process.on(sig, async () => {
    console.error(`\n${sig}: cleaning up`);
    await cleanup();
    process.exit(130);
  });
}
process.on('exit', () => {
  for (const c of children) {
    if (c.pid) {
      try {
        process.kill(-c.pid, 'SIGKILL');
      } catch {}
    }
  }
  for (const d of tmpdirs) rmSync(d, { recursive: true, force: true });
});

async function startTrunk() {
  const port = await freePort();
  const dist = mkdtempSync(join(tmpdir(), 'nocomponents-dist-'));
  tmpdirs.push(dist);
  const t = launch(
    'trunk',
    ['serve', '--port', String(port), '--address', '127.0.0.1', '--dist', dist, '--no-autoreload', '--disable-address-lookup'],
    { cwd: docsDir },
  );
  const origin = `http://127.0.0.1:${port}`;
  console.log(`trunk: building into ${dist}, serving ${origin} (the first build takes minutes)`);
  const minutes = Number(process.env.SMOKE_BUILD_TIMEOUT_MIN) || 10;
  const end = Date.now() + minutes * 60 * 1000;
  let last = Date.now();
  for (;;) {
    if (t.spawnError) throw new Error(`could not start trunk: ${t.spawnError.message}`);
    if (t.exitCode !== null) throw new Error(`trunk exited (${t.exitCode}):\n${t.out}`);
    const built =
      existsSync(join(dist, 'index.html')) && readdirSync(dist).some((f) => f.endsWith('.wasm'));
    if (built) {
      const r = await fetch(`${origin}/docs`).catch(() => null);
      if (r?.status === 200) return origin;
    }
    if (Date.now() > end) throw new Error(`trunk did not finish within ${minutes} minutes:\n${t.out}`);
    if (Date.now() - last > 30000) {
      console.log('trunk: still building...');
      last = Date.now();
    }
    await sleep(500);
  }
}

function findChrome() {
  if (process.env.CHROME) return process.env.CHROME;
  const path = (process.env.PATH ?? '').split(':');
  for (const name of ['chromium', 'chromium-browser', 'google-chrome', 'google-chrome-stable']) {
    for (const d of path) if (existsSync(join(d, name))) return join(d, name);
  }
  throw new Error('no chromium found on PATH (tried chromium, chromium-browser, google-chrome); set CHROME');
}

// A cold Chrome on a shared CI runner can take well over the 20 s a warm one needs, so the wait is
// configurable and a launch that never opens its port is retried once with a fresh profile.
const CHROME_START_SEC = Number(process.env.SMOKE_CHROME_START_SEC);
const CHROME_START_MS = (Number.isFinite(CHROME_START_SEC) && CHROME_START_SEC > 0 ? CHROME_START_SEC : 20) * 1000;

async function startChrome(attempt = 1) {
  const port = await freePort();
  const profile = mkdtempSync(join(tmpdir(), 'nocomponents-chrome-'));
  tmpdirs.push(profile);
  const argv = [
    '--headless=new',
    `--remote-debugging-port=${port}`,
    `--user-data-dir=${profile}`,
    '--no-first-run',
    '--no-default-browser-check',
    // A container's /dev/shm is small; Chrome's shared memory there is what stalls or crashes it.
    '--disable-dev-shm-usage',
    '--disable-gpu',
    'about:blank',
  ];
  if (process.env.CHROME_NO_SANDBOX) argv.unshift('--no-sandbox');
  const c = launch(findChrome(), argv);
  try {
    return await Browser.connect(port, CHROME_START_MS);
  } catch (e) {
    if (attempt < 2) {
      console.log(`chrome: no debugging port after ${CHROME_START_MS / 1000}s, relaunching`);
      try {
        if (c.pid) process.kill(-c.pid, 'SIGKILL');
      } catch {}
      return startChrome(attempt + 1);
    }
    throw new Error(`${e.message}\n${c.out}`);
  }
}

async function loadChecks() {
  const dir = join(here, 'checks');
  const out = [];
  for (const f of readdirSync(dir).filter((f) => f.endsWith('.mjs')).sort()) {
    const name = f.replace(/\.mjs$/, '');
    const mod = await import(pathToFileURL(join(dir, f)).href);
    out.push({ name, run: mod.default, skip: mod.skip, path: mod.path ?? `/docs/${name}` });
  }
  return out;
}

const started = Date.now();
const failures = [];
const skipped = [];
let ran = 0;
let swept = 0;

async function main() {
  const origin = value('--url')?.replace(/\/$/, '') ?? (await startTrunk());
  const browser = await startChrome();

  if (!flag('--no-sweep')) {
    const list = pages().filter(wanted);
    swept = list.length;
    const t0 = Date.now();
    const f = await sweep(browser, origin, list);
    failures.push(...f);
    console.log(`sweep: ${list.length} pages, ${f.length} problem(s) (${((Date.now() - t0) / 1000).toFixed(1)}s)`);
  }

  const checks = (await loadChecks()).filter((c) => wanted(c.name));
  for (const c of checks) {
    if (c.skip) {
      skipped.push({ name: c.name, reason: c.skip });
      console.log(`  skip  ${c.name}: ${c.skip}`);
      continue;
    }
    const t = await browser.page(origin);
    t.label = c.path;
    const t0 = Date.now();
    try {
      await t.goto(c.path);
      await c.run(t);
      await sleep(150);
    } catch (e) {
      if (e instanceof Skip) {
        skipped.push({ name: c.name, reason: e.message });
        console.log(`  skip  ${c.name}: ${e.message}`);
        await t.close();
        continue;
      }
      t.fail('check threw', e.stack?.split('\n').slice(0, 3).join(' | ') ?? String(e));
    }
    for (const p of t.consoleProblems) t.fail('console', p);
    ran++;
    console.log(`  ${t.failures.length ? 'FAIL' : 'ok  '}  ${c.name} (${Date.now() - t0}ms)`);
    failures.push(...t.failures);
    await t.close();
  }
}

let crashed = null;
try {
  await main();
} catch (e) {
  crashed = e;
}
await cleanup();

const secs = ((Date.now() - started) / 1000).toFixed(1);
if (crashed) {
  console.error(`\nrunner error: ${crashed.message}`);
  process.exit(2);
}
console.log('');
for (const f of failures) console.log(`FAIL ${f.page}: ${f.label}: ${f.detail}`);
console.log(
  `${failures.length ? 'FAILED' : 'passed'}: ${swept} pages swept, ${ran} checks run, ${skipped.length} skipped, ${failures.length} failure(s) in ${secs}s`,
);
for (const s of skipped) console.log(`skipped ${s.name}: ${s.reason}`);
process.exit(failures.length ? 1 : 0);
