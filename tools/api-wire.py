#!/usr/bin/env python3
"""Wire a drafted API const into a docs page: import, const, and the <ApiReference/> call."""
import sys, pathlib

page = pathlib.Path(sys.argv[1])
api = pathlib.Path(sys.argv[2]).read_text().rstrip() + "\n"
s = page.read_text()

IMPORT_OLD = "use crate::{components::demo_section::DemoSection, layout::doc_layout::DocLayout};"
IMPORT_NEW = """use crate::{
    components::{
        api_table::{ApiEntry, ApiReference, Prop},
        demo_section::DemoSection,
    },
    layout::doc_layout::DocLayout,
};"""

if "api_table" not in s:
    assert IMPORT_OLD in s, f"{page}: unexpected import shape"
    s = s.replace(IMPORT_OLD, IMPORT_NEW)

marker = "\n#[component]\npub fn Page("
assert marker in s, f"{page}: no Page component"
s = s.replace(marker, "\n" + api + marker, 1)

tail = """            </div>
        </DocLayout>
    }
}"""
assert s.count(tail) == 1, f"{page}: {s.count(tail)} tails"
s = s.replace(tail, """
                <ApiReference entries=API />
            </div>
        </DocLayout>
    }
}""")
page.write_text(s)
print(f"wired {page}")
