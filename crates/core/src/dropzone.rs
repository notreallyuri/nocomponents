//! File type filtering for dropzones and file pickers.

/// Whether a file is one of the kinds `accept` asks for.
///
/// The same grammar the `accept` attribute uses, because a caller should not have to learn a
/// second one: a comma-separated list of `.ext`, `type/subtype`, or `type/*`. An empty list
/// accepts everything, which is what the attribute means by being absent.
///
/// Applied to both ways in. The attribute decides what a file dialog *offers*, not what it hands
/// back — the reader can switch it to "All files" — and a drop does not consult it at all.
/// Checking here is what makes `accept` a rule rather than a suggestion.
pub fn accepts(accept: &str, mime: &str, name: &str) -> bool {
    let accept = accept.trim();
    if accept.is_empty() {
        return true;
    }

    let mime = mime.to_ascii_lowercase();
    let name = name.to_ascii_lowercase();

    accept.split(',').map(str::trim).any(|pattern| {
        let pattern = pattern.to_ascii_lowercase();
        if pattern.is_empty() {
            return false;
        }
        if let Some(extension) = pattern.strip_prefix('.') {
            return name.ends_with(&format!(".{extension}"));
        }
        if let Some(family) = pattern.strip_suffix("/*") {
            return mime.split('/').next() == Some(family);
        }
        mime == pattern
    })
}

#[cfg(test)]
mod tests {
    use super::accepts;

    #[test]
    fn an_empty_accept_takes_anything() {
        assert!(accepts("", "image/png", "a.png"));
        assert!(accepts("  ", "application/x-thing", "a.bin"));
    }

    #[test]
    fn a_family_matches_its_whole_type() {
        assert!(accepts("image/*", "image/png", "a.png"));
        assert!(accepts("image/*", "image/svg+xml", "a.svg"));
        assert!(!accepts("image/*", "video/mp4", "a.mp4"));
    }

    #[test]
    fn an_exact_type_matches_only_itself() {
        assert!(accepts("application/pdf", "application/pdf", "a.pdf"));
        assert!(!accepts("application/pdf", "application/json", "a.json"));
    }

    #[test]
    fn an_extension_matches_the_name_a_drop_arrives_with() {
        // A dropped file often has no type at all, which is why the name is checked too.
        assert!(accepts(".pdf", "", "report.pdf"));
        assert!(accepts(".PDF", "", "REPORT.pdf"));
        assert!(!accepts(".pdf", "", "report.pdf.exe"));
        assert!(!accepts(".pdf", "", "pdf"));
    }

    #[test]
    fn a_list_takes_any_of_its_entries() {
        assert!(accepts("image/*, .pdf", "image/gif", "a.gif"));
        assert!(accepts("image/*, .pdf", "", "a.pdf"));
        assert!(!accepts("image/*, .pdf", "text/csv", "a.csv"));
    }
}
