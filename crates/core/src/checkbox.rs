//! Pure checkbox state transitions.

/// What a click makes of the box. A box showing "mixed" is checked by the click — whatever its
/// `checked` held underneath — and anything else flips.
pub fn next_checked(checked: bool, indeterminate: bool) -> bool {
    indeterminate || !checked
}

/// The state a checkbox shows, which is `data-state` and decides `aria-checked`.
pub fn checkbox_state(checked: bool, indeterminate: bool) -> &'static str {
    if indeterminate {
        "indeterminate"
    } else if checked {
        "checked"
    } else {
        "unchecked"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_click_flips_a_plain_box() {
        assert!(next_checked(false, false));
        assert!(!next_checked(true, false));
    }

    #[test]
    fn a_click_checks_an_indeterminate_box_whatever_it_held() {
        assert!(next_checked(false, true));
        assert!(next_checked(true, true));
    }

    #[test]
    fn indeterminate_wins_the_state() {
        assert_eq!(checkbox_state(true, true), "indeterminate");
        assert_eq!(checkbox_state(true, false), "checked");
        assert_eq!(checkbox_state(false, false), "unchecked");
    }
}
