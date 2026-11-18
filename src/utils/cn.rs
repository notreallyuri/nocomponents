/// Joins classes and runs them through `tw_merge`, so a caller's class can override a component's.
///
/// The `tw_merge` path goes through `$crate::deps`: the styled layer is *vendored into* a consumer's
/// project, where `tw_merge` is not a dependency and a bare `tw_merge::` would not resolve.
/// Reaching it through this crate means the consumer needs one dependency, not two, and cannot end
/// up on a different version of the merger than the classes were written against.
#[macro_export]
macro_rules! cn {
    ($($class:expr),+ $(,)?) => {
        {
            let combined = vec![$($class.to_string()),+].join(" ");
            $crate::deps::tw_merge::tw_merge!(&combined)
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_callers_size_replaces_the_icon_variable_default() {
        assert_eq!(
            crate::cn!("size-[var(--icon-size,1rem)]", "size-6"),
            "size-6"
        );
        assert_eq!(
            crate::cn!("size-[var(--icon-size,1rem)]", "size-6 text-red-500"),
            "size-6 text-red-500"
        );
        assert_eq!(
            crate::cn!(
                "size-[var(--icon-size,1em)]",
                "size-[var(--icon-size,1rem)]"
            ),
            "size-[var(--icon-size,1rem)]"
        );
    }

    #[test]
    fn a_size_variant_replaces_the_base_icon_variable() {
        assert_eq!(
            crate::cn!("[--icon-size:1rem] px-2", "[--icon-size:0.75rem]"),
            "px-2 [--icon-size:0.75rem]"
        );
    }
}
