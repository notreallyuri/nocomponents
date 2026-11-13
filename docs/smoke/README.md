# Browser smoke suite

Drives headless Chromium over the DevTools protocol against the docs site. No dependencies and no
`package.json`: Node 24's built-in `WebSocket` and `fetch`, plus `node:child_process` and
`node:fs`.

```bash
node docs/smoke/run.mjs                       # build the site, sweep every page, run every check
node docs/smoke/run.mjs --url http://127.0.0.1:3000   # use a server that is already running
node docs/smoke/run.mjs --only dropdown       # only checks / pages whose name contains the text
node docs/smoke/run.mjs --no-sweep            # checks only
```

It exits non-zero on any failure, with a `FAIL <page>: <label>: <detail>` line for each.

Without `--url` it starts its own `trunk serve` on a free port into a temporary `--dist`, so it never
fights a `trunk serve` of yours on :3000. The first build takes minutes; later ones reuse `target/`. `SMOKE_BUILD_TIMEOUT_MIN` raises the
10-minute limit on that build.
It then starts `chromium` (first of `chromium`, `chromium-browser`, `google-chrome` on `PATH`;
`CHROME=/path/to/binary` overrides; `CHROME_NO_SANDBOX=1` adds `--no-sandbox`, which a root-run
container needs). Both are killed by process group on exit, failure and SIGINT.

## What runs

- **`sweep.mjs`** loads every page and fails on a console error or warning, an uncaught exception,
  or a "Not Found" `<main>`. The list is read from the source: the `/docs/...` hrefs of `NAV` in
  `docs/src/layout/doc_layout.rs` (except `/docs/introduction`, a known dead link) and
  `/blocks/<slug>` for every `slug:` under `docs/src/pages/docs/blocks/`.
- **`checks/<page>.mjs`** is one interaction check per docs page; the file name is the page
  (`checks/alert-dialog.mjs` runs on `/docs/alert-dialog`). Console problems during a check fail it
  too.

## Adding a check

```js
import { demo, demoBtn } from '../cdp.mjs';

export default async (t) => {
  const trig = `${demo('[data-slot=popover-trigger]')}[0]`;
  await t.click(trig, 500);
  t.expect('opens', await t.attr(trig, 'aria-expanded'), 'true');
  await t.settle('closes on Escape', `${trig}.getAttribute('aria-expanded')`, 'false');
};
```

The page is already loaded and has a `[data-slot]` when the check starts. Selectors are JavaScript
*expressions* in strings that evaluate to an element; `cdp.mjs` has builders (`q`, `qa`, `demo`,
`byText`, `demoBtn`, `menuItem`, `section`). The driver: `ev`, `click`, `hover`, `key`, `type`,
`focus`, `clickAt`, `viewport`, `goto`, `shot`, `attr`, `text`, `exists`, `waitFor`. Assertions:
`expect(label, actual, expected)`, `ok(label, cond)` and `settle(label, expr, expected)`, which polls
instead of sleeping. They record a failure and carry on, so one run reports everything wrong.

A module may `export const path = '/docs/...'` to run somewhere else, and `export const skip =
'reason'` to switch itself off: the runner prints the reason. Use `skip` for a real library bug that
is waiting on a fix, not to quiet a flaky check.

## Selector pitfalls

- `main` **contains the site header**, which has a theme-switcher popover (`System`,
  `data-slot=popover-trigger`, `aria-haspopup=dialog`) and a sidebar trigger. A bare `main button`
  or `main [data-slot=popover-trigger]` finds those first. Use `demo(css)` (which skips `header`),
  `section(/title/i)`, or match on text.
- Menus, selects and dialogs render in a portal, outside `main`: search `document` for those.
- After choosing a date the first "Pick a date" trigger's text changes, so tag it
  (`dataset.smoke`) before clicking and find it by that.
- The tree writes no `aria-selected`; its demo prints `Selected: <name>`.
- The colour picker's format control is a `Select`, not a cycling button.
- A key event needs `text` for the browser to activate a focused button on Enter; `key('Enter')`
  does that.
- Headless has no clipboard, so the copy button's success path is not asserted, and CDP caps
  synthetic pointer speed, so there are no flick or velocity tests.
- Use `127.0.0.1`, not `localhost`.
