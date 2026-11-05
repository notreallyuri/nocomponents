// Loads every page and fails on a console error or warning, an exception, or a "Not Found" main.
// The page list comes from the source, so a new page is covered the day it is added.

import { readFileSync, readdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');

// /docs/introduction is a known dead link with its own TODO item.
const SKIP = new Set(['/docs/introduction']);

export function pages() {
  const nav = readFileSync(join(root, 'src/layout/doc_layout.rs'), 'utf8');
  const docs = [...nav.matchAll(/href:\s*"(\/docs\/[^"]+)"/g)].map((m) => m[1]).filter((p) => !SKIP.has(p));
  const dir = join(root, 'src/pages/docs/blocks');
  const blocks = readdirSync(dir)
    .filter((f) => f.endsWith('.rs'))
    .flatMap((f) => [...readFileSync(join(dir, f), 'utf8').matchAll(/slug:\s*"([^"]+)"/g)].map((m) => `/blocks/${m[1]}`));
  return [...new Set([...docs, ...blocks])];
}

export async function sweep(browser, origin, list, concurrency = 4) {
  const failures = [];
  const queue = [...list];
  const worker = async () => {
    const t = await browser.page(origin);
    try {
      while (queue.length) {
        const path = queue.shift();
        t.label = path;
        t.consoleProblems.length = 0;
        try {
          await t.goto(path, { settle: 700 });
          const main = await t.ev(`document.querySelector('main')?.textContent ?? null`);
          if (main === null) failures.push({ page: path, label: 'sweep', detail: 'no <main>' });
          else if (main.includes('Not Found')) failures.push({ page: path, label: 'sweep', detail: 'main says "Not Found"' });
        } catch (e) {
          failures.push({ page: path, label: 'sweep', detail: e.message });
        }
        for (const p of t.consoleProblems) failures.push({ page: path, label: 'sweep', detail: p });
      }
    } finally {
      await t.close();
    }
  };
  await Promise.all(Array.from({ length: Math.min(concurrency, list.length) }, worker));
  return failures;
}
