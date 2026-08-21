#!/usr/bin/env python3
"""Fill a draft's TODO descriptions from `Key = text` lines on stdin.

Keys are `Component` for a part's own description and `Component.prop` for a prop's, so the fills
are matched by name rather than by order — a description that came from a doc comment is simply
not a key, and a typo is an error rather than a silent shift.
"""
import sys, pathlib, re

def rust_str(v):
    return '"' + v.replace("\\", "\\\\").replace('"', '\\"') + '"'

draft = pathlib.Path(sys.argv[1])
lines = draft.read_text().splitlines()
fills = {}
for line in sys.stdin.read().splitlines():
    if not line.strip(): continue
    key, _, text = line.partition("=")
    fills[key.strip()] = text.strip()

used, missing, current = set(), [], None
for i, line in enumerate(lines):
    m = re.search(r'name: "(\w+)",$', line.strip())
    if m and "ApiEntry" not in line:
        pass
    if re.match(r'\s*name: "(\w+)",$', line):
        current = re.match(r'\s*name: "(\w+)",$', line).group(1)
        continue
    if 'description: "TODO"' in line and "Prop {" not in line:
        key = current
    elif 'description: "TODO"' in line:
        key = f'{current}.{re.search(r"""Prop \{ name: "(\w+)""", line).group(1)}'
    else:
        continue
    if key in fills:
        lines[i] = line.replace('description: "TODO"', "description: " + rust_str(fills[key]))
        used.add(key)
    else:
        missing.append(key)

unknown = set(fills) - used
assert not missing, f"{draft.name}: no fill for {missing}"
assert not unknown, f"{draft.name}: unused keys {sorted(unknown)}"
draft.write_text("\n".join(lines) + "\n")
print(f"filled {draft.name} ({len(used)})")
