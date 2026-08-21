#!/usr/bin/env python3
"""Draft `ApiEntry` rows from a component file's `#[component]` signatures.

Names, types and defaults come from the source; descriptions come from the `///` above each prop
where there is one, and are left as TODO where there is not.
"""
import re, sys, pathlib

def enum_index():
    """name -> (variants, default variant), so an enum prop's default is the real one."""
    out = {}
    for f in list(pathlib.Path("src").rglob("*.rs")):
        src = f.read_text()
        for m in re.finditer(r"pub enum (\w+) \{(.*?)\n\}", src, re.S):
            name, body = m.group(1), m.group(2)
            variants, default = [], None
            lines = [l.strip() for l in body.splitlines()]
            marked = False
            for l in lines:
                if l.startswith("#[default]"):
                    marked = True
                    continue
                if l.startswith("//") or l.startswith("#["): continue
                v = re.match(r"(\w+)", l)
                if v:
                    variants.append(v.group(1))
                    if marked: default, marked = v.group(1), False
            if variants:
                out[name] = (variants, default or variants[0])
    return out

ENUMS = enum_index()

def split_args(s):
    """Split a signature on top-level commas, ignoring the ones inside comments and brackets."""
    out, depth, cur, i = [], 0, "", 0
    while i < len(s):
        ch = s[i]
        if s.startswith("//", i):
            end = s.find("\n", i)
            end = len(s) if end == -1 else end
            cur += s[i:end]
            i = end
            continue
        if ch in "<([{": depth += 1
        elif ch in ">)]}": depth -= 1
        if ch == "," and depth == 0:
            out.append(cur); cur = ""
        else:
            cur += ch
        i += 1
    if cur.strip(): out.append(cur)
    return out

def default_for(ty, attr):
    m = re.search(r"default\s*=\s*(.+?)(?:,\s*into)?\s*\)?$", attr.replace("#[prop(", "").rstrip("]"))
    if "default =" in attr:
        # bracket-matched, so `100.0.into()` and `Side::Bottom` both come out whole
        start = attr.index("default =") + len("default =")
        depth, v = 0, ""
        for ch in attr[start:]:
            if ch in "([{<": depth += 1
            elif ch in ")]}>":
                if depth == 0: break
                depth -= 1
            elif ch == "," and depth == 0: break
            v += ch
        v = v.strip()
        if v: return v
    if ty.startswith("Option<"): return "None"
    if "optional" in attr:
        if "Signal<String>" in ty: return '\\"\\"'
        if "Signal<bool>" in ty: return "false"
        if ty.startswith("Signal<Option<"): return "None"
        if ty == "Signal<f64>": return "0.0"
        if ty in ("f64", "u32", "usize", "i32"): return "0"
        if ty in ENUMS: return ENUMS[ty][1]
        if ty == "SideOffset": return "SideOffset(4.0)"
        # an optional signal prop is created for the caller, holding its type's default
        if ty.startswith("RwSignal<Option<"): return "None"
        if ty == "RwSignal<bool>": return "false"
        if ty.startswith("RwSignal<Vec<"): return "empty"
        if ty.startswith("RwSignal<String"): return '\\"\\"' 
        if ty == "String": return '\\"\\"'
        return "TODO"
    return ""

def props_of(sig, docs):
    rows = []
    for raw in split_args(sig):
        arg = raw.strip()
        if not arg: continue
        doc = []
        attr = ""
        lines = [l.strip() for l in arg.splitlines() if l.strip()]
        decl = []
        for l in lines:
            if l.startswith("///"):
                doc.append(l[3:].strip())
                continue
            # an attribute and the declaration usually share a line: peel the attributes off the
            # front, bracket-matched, and whatever is left is the declaration.
            while l.startswith("#["):
                depth, i = 0, 0
                for i, ch in enumerate(l):
                    if ch == "[": depth += 1
                    elif ch == "]":
                        depth -= 1
                        if depth == 0: break
                attr += l[: i + 1]
                l = l[i + 1 :].strip()
            if l: decl.append(l)
        decl = " ".join(decl)
        if ":" not in decl: continue
        name, ty = decl.split(":", 1)
        name, ty = name.strip(), ty.strip().rstrip(",")
        ty = re.sub(r"\s+", " ", ty).replace("< ", "<").replace(" >", ">").replace(", >", ">")
        ty = ty.replace(",>", ">")
        if not name or name.startswith("#"): continue
        rows.append((name, ty, default_for(ty, attr), " ".join(doc)))
    return rows

def rs(v):
    """A doc comment becomes a Rust string literal, so its quotes have to survive the trip."""
    return v.replace("\\", "\\\\").replace('"', '\\"')

def main(path):
    src = pathlib.Path(path).read_text()
    out = []
    for m in re.finditer(r"#\[component\]\s*\npub fn (\w+)\(\n(.*?)\n\) -> impl IntoView", src, re.S):
        name, sig = m.group(1), m.group(2)
        # the doc comment above the component, if any
        head = src[:m.start()].rstrip().splitlines()
        desc = []
        for line in reversed(head):
            if line.strip().startswith("///"): desc.insert(0, line.strip()[3:].strip())
            else: break
        out.append((m.start(), name, " ".join(desc), props_of(sig, None)))
    # single-line components, e.g. `pub fn Skeleton(a: T) -> impl IntoView`
    for m in re.finditer(r"#\[component\]\s*\npub fn (\w+)\((.*?)\) -> impl IntoView", src, re.S):
        if any(o[1] == m.group(1) for o in out): continue
        out.append((m.start(), m.group(1), "", props_of(m.group(2), None)))
    out.sort(key=lambda o: o[0])

    print("const API: &[ApiEntry] = &[")
    for _, name, desc, rows in out:
        print(f'    ApiEntry {{')
        print(f'        name: "{name}",')
        print(f'        description: "{rs(desc) if desc else "TODO"}",')
        print(f'        props: &[')
        for n, t, d, doc in rows:
            if not doc and t in ENUMS:
                doc = "One of: " + ", ".join(ENUMS[t][0]) + "."
            print(f'            Prop {{ name: "{n}", ty: "{t}", default: "{d}", description: "{rs(doc) if doc else "TODO"}" }},')
        print(f'        ],')
        print(f'    }},')
    print("];")

main(sys.argv[1])
