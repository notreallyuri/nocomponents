use tw_merge::tw_merge;

pub fn cn(base: &str, extra: &str) -> String {
    tw_merge!(base, extra)
}
