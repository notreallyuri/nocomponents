//! Rewrites a file from the library's `src/components/` so it compiles inside another crate.
//!
//! Imports are nested trees (`use crate::{cn, components::button::…, primitives::…}`), so
//! `crate::components::` is not a substring of what has to change. Each `use` is flattened to one
//! full path per line, rewritten, and printed back.

pub struct Rewritten {
    pub source: String,
    pub component_deps: Vec<String>,
    pub uses_icons: bool,
}

struct UseStatement {
    start: usize,
    end: usize,
    is_pub: bool,
    paths: Vec<String>,
}

pub fn rewrite(source: &str, module_path: &str) -> Rewritten {
    let uses = find_uses(source);

    let mut component_deps = Vec::new();
    let mut uses_icons = false;
    let mut out = String::with_capacity(source.len());
    let mut cursor = 0;

    for u in &uses {
        out.push_str(&rewrite_body(&source[cursor..u.start], module_path));

        let mut lines: Vec<String> = Vec::new();
        for path in &u.paths {
            note_dep(path, &mut component_deps, &mut uses_icons);
            let keyword = if u.is_pub { "pub use" } else { "use" };
            lines.push(format!("{keyword} {};", rewrite_path(path, module_path)));
        }
        lines.sort();
        out.push_str(&lines.join("\n"));

        cursor = u.end;
    }
    out.push_str(&rewrite_body(&source[cursor..], module_path));

    // Paths in expression position, which the import scan cannot see.
    for path in inline_paths(source) {
        note_dep(&path, &mut component_deps, &mut uses_icons);
    }
    component_deps.sort();
    component_deps.dedup();

    Rewritten {
        source: out,
        component_deps,
        uses_icons,
    }
}

fn note_dep(path: &str, deps: &mut Vec<String>, uses_icons: &mut bool) {
    if let Some(rest) = path.strip_prefix("crate::components::")
        && let Some(name) = rest.split("::").next()
        && !name.is_empty()
    {
        deps.push(name.to_string());
    }
    if path.starts_with("crate::icons") {
        *uses_icons = true;
    }
}

fn rewrite_path(path: &str, module_path: &str) -> String {
    if let Some(rest) = path.strip_prefix("crate::components::") {
        format!("{module_path}::{rest}")
    } else if let Some(rest) = path.strip_prefix("crate::") {
        format!("nocomponents::{rest}")
    } else if path == "leptos_node_ref" || path.starts_with("leptos_node_ref::") {
        format!("nocomponents::deps::{path}")
    } else {
        path.to_string()
    }
}

fn rewrite_body(body: &str, module_path: &str) -> String {
    body.replace("crate::components::", &format!("{module_path}::"))
        .replace("crate::primitives::", "nocomponents::primitives::")
        .replace("crate::icons::", "nocomponents::icons::")
        .replace("crate::utils::", "nocomponents::utils::")
        .replace("js_sys::", "nocomponents::deps::js_sys::")
}

fn inline_paths(source: &str) -> Vec<String> {
    let mut found = Vec::new();
    for prefix in ["crate::components::", "crate::icons::"] {
        let mut rest = source;
        while let Some(i) = rest.find(prefix) {
            let tail = &rest[i + prefix.len()..];
            let name: String = tail
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            found.push(format!("{prefix}{name}"));
            rest = &rest[i + prefix.len()..];
        }
    }
    found
}

/// Every `use` / `pub use` statement, with its tree flattened to full paths. A statement starts at
/// a line beginning with the keyword and ends at the first `;` outside a brace group.
fn find_uses(source: &str) -> Vec<UseStatement> {
    let bytes = source.as_bytes();
    let mut out = Vec::new();
    let mut line_start = 0;

    while line_start < source.len() {
        let line_end = source[line_start..]
            .find('\n')
            .map(|i| line_start + i)
            .unwrap_or(source.len());
        let line = &source[line_start..line_end];
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();

        let (is_pub, keyword_len) = if trimmed.starts_with("pub use ") {
            (true, 8)
        } else if trimmed.starts_with("use ") {
            (false, 4)
        } else {
            line_start = line_end + 1;
            continue;
        };

        let body_start = line_start + indent + keyword_len;
        let mut depth = 0usize;
        let mut i = body_start;
        while i < bytes.len() {
            match bytes[i] {
                b'{' => depth += 1,
                b'}' => depth = depth.saturating_sub(1),
                b';' if depth == 0 => break,
                _ => {}
            }
            i += 1;
        }
        let semi = i.min(bytes.len());

        out.push(UseStatement {
            start: line_start + indent,
            end: (semi + 1).min(source.len()),
            is_pub,
            paths: flatten(&source[body_start..semi], ""),
        });
        line_start = semi + 1;
    }
    out
}

/// `a::{b, c::{d, e}}` becomes `a::b`, `a::c::d`, `a::c::e`.
fn flatten(body: &str, prefix: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut current = String::new();

    for ch in body.chars() {
        match ch {
            '{' => {
                depth += 1;
                current.push(ch);
            }
            '}' => {
                depth = depth.saturating_sub(1);
                current.push(ch);
            }
            ',' if depth == 0 => {
                expand(&current, prefix, &mut out);
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    expand(&current, prefix, &mut out);
    out
}

fn expand(item: &str, prefix: &str, out: &mut Vec<String>) {
    let item = item.trim();
    if item.is_empty() {
        return;
    }
    match item.split_once('{') {
        Some((head, rest)) => {
            let inner = rest.strip_suffix('}').unwrap_or(rest);
            let prefix = format!("{prefix}{}", head.trim());
            out.extend(flatten(inner, &prefix));
        }
        None => {
            // Collapse the whitespace a multi-line tree leaves inside `X as Y`.
            let leaf = item.split_whitespace().collect::<Vec<_>>().join(" ");
            out.push(format!("{prefix}{leaf}"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MP: &str = "crate::components::nc";

    fn rw(src: &str) -> Rewritten {
        rewrite(src, MP)
    }

    #[test]
    fn flattens_a_nested_tree_and_redirects_each_leaf() {
        let out = rw(
            "use crate::{\n    cn,\n    components::button::{Button, ButtonSize},\n    primitives::combobox::ComboboxRoot,\n};\n",
        );
        assert!(out.source.contains("use nocomponents::cn;"));
        assert!(
            out.source
                .contains("use crate::components::nc::button::Button;")
        );
        assert!(
            out.source
                .contains("use crate::components::nc::button::ButtonSize;")
        );
        assert!(
            out.source
                .contains("use nocomponents::primitives::combobox::ComboboxRoot;")
        );
        assert_eq!(out.component_deps, vec!["button"]);
    }

    #[test]
    fn keeps_a_rename_and_the_pub_of_a_re_export() {
        let out = rw("pub use crate::components::sheet::{SheetClose as DrawerClose};\n");
        assert!(
            out.source
                .contains("pub use crate::components::nc::sheet::SheetClose as DrawerClose;")
        );
        assert_eq!(out.component_deps, vec!["sheet"]);
    }

    #[test]
    fn joins_a_rename_split_over_two_lines() {
        let out = rw("use crate::icons::calendar::{\n    Calendar as\n    CalendarIcon,\n};\n");
        assert!(
            out.source
                .contains("use nocomponents::icons::calendar::Calendar as CalendarIcon;")
        );
        assert!(out.uses_icons);
    }

    #[test]
    fn leaves_leptos_alone_but_reroutes_the_node_ref() {
        let out = rw("use leptos::prelude::*;\nuse leptos_node_ref::AnyNodeRef;\n");
        assert!(out.source.contains("use leptos::prelude::*;"));
        assert!(
            out.source
                .contains("use nocomponents::deps::leptos_node_ref::AnyNodeRef;")
        );
    }

    #[test]
    fn redirects_a_path_written_in_expression_position() {
        let out = rw("fn f() { let n = js_sys::Date::now(); }\n");
        assert!(
            out.source
                .contains("nocomponents::deps::js_sys::Date::now()")
        );
    }

    #[test]
    fn finds_a_component_named_only_in_the_body() {
        let out = rw("fn f() { crate::components::separator::Separator(); }\n");
        assert_eq!(out.component_deps, vec!["separator"]);
        assert!(
            out.source
                .contains("crate::components::nc::separator::Separator()")
        );
    }

    #[test]
    fn leaves_a_body_that_names_nothing_of_ours_untouched() {
        let src = "#[component]\npub fn Card() -> impl IntoView {\n    view! { <div class=\"p-4\" /> }\n}\n";
        assert_eq!(rw(src).source, src);
    }
}
