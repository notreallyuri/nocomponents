#[macro_export]
macro_rules! cn {
    ($($class:expr),+ $(,)?) => {
        {
            let combined = vec![$($class.to_string()),+].join(" ");
            tw_merge::tw_merge!(&combined)
        }
    };
}
