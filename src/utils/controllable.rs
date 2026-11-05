use leptos::prelude::*;

/// Resolves the three props a stateful root offers for one piece of state — `x`, `default_x` and
/// `on_x_change` — into the one signal the root works with.
///
/// - `value` is the caller's signal. When given it is returned as is and `default` is ignored,
///   so the caller can read and write it exactly as before.
/// - `default` is the starting value for a caller who gave no signal and only wants a place to
///   begin.
/// - `on_change` is for a caller who only wants to be told. It runs after the value changes,
///   never for the starting value, and not when a write leaves the value equal to what it was.
///
/// It fires on *any* change, including one the caller made through their own signal, not only
/// the ones the component made. That is deliberate: a callback that heard only about
/// the component's own writes would need every `.set()` in the root to go through it, and the
/// ones that already exist keep working untouched this way.
///
/// Call it once, in the root, where there is an owner for the effect to live under.
pub fn controllable<T>(
    value: Option<RwSignal<T>>,
    default: T,
    on_change: Option<Callback<T>>,
) -> RwSignal<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let signal = value.unwrap_or_else(|| RwSignal::new(default));

    if let Some(on_change) = on_change {
        Effect::new(move |previous: Option<T>| {
            let current = signal.get();

            if let Some(previous) = &previous
                && *previous != current
            {
                on_change.run(current.clone());
            }

            current
        });
    }

    signal
}

/// Hands an `Option<Callback<T>>` on to a part whose own prop is `#[prop(optional, into)]`.
///
/// That declaration makes the setter take the `Callback<T>` itself, so an `Option` held by the
/// forwarding component cannot be passed as it stands. `None` becomes a callback that does
/// nothing, which is what the receiver would have done without one.
pub fn forward<T: 'static>(callback: Option<Callback<T>>) -> Callback<T> {
    callback.unwrap_or_else(|| Callback::new(|_| {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_given_signal_is_returned_and_the_default_ignored() {
        let owner = Owner::new();
        owner.with(|| {
            let mine = RwSignal::new(true);
            let resolved = controllable(Some(mine), false, None);

            assert!(resolved.get_untracked());
            resolved.set(false);
            assert!(!mine.get_untracked());
        });
    }

    #[test]
    fn without_a_signal_the_default_is_the_start() {
        let owner = Owner::new();
        owner.with(|| {
            assert!(controllable(None, true, None).get_untracked());
            assert!(!controllable(None, false, None).get_untracked());
        });
    }
}
