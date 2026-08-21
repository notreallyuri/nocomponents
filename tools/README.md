# tools

Throwaway-quality scripts that drafted the API reference tables in `docs/`. They are kept because
they are the prototype for generating those tables at build time — see the TODO — not because
anything runs them automatically.

```bash
python3 tools/api-extract.py src/components/badge.rs > /tmp/badge.rs   # draft the ApiEntry rows
python3 tools/api-fill.py /tmp/badge.rs < fills.txt                    # fill the TODO descriptions
python3 tools/api-wire.py docs/src/pages/docs/badge.rs /tmp/badge.rs   # insert into the page
```

`api-extract.py` reads one `src/components/*.rs`: prop names, types and defaults come from the
`#[component]` signature, descriptions from a `///` above a prop or a component where there is one,
and enum variants from the enum's own definition. Everything else it leaves as `TODO`.

`api-fill.py` takes `Component = text` and `Component.prop = text` lines and refuses to run if a
key is missing or unknown, so a description cannot silently land on the wrong row.
