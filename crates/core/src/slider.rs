//! Pure slider arithmetic and headless multi-thumb [`SliderState`].

/// How many decimals `step` is written to, so snapping does not turn 0.3 into 0.30000000000000004.
pub fn decimals_of(step: f64) -> i32 {
    match format!("{step}").split_once('.') {
        Some((_, frac)) => frac.len() as i32,
        None => 0,
    }
}

/// Clamps `value` to `[min, max]` and snaps it to the nearest `step` from `min`.
pub fn clamp_and_step(value: f64, min: f64, max: f64, step: f64) -> f64 {
    if max <= min {
        return min;
    }
    if step <= 0.0 {
        return value.clamp(min, max);
    }

    let steps = ((value - min) / step).round();
    let snapped = (min + steps * step).clamp(min, max);
    let factor = 10f64.powi(decimals_of(step));

    ((snapped * factor).round() / factor).clamp(min, max)
}

/// Where `value` sits along `[min, max]`, as `0.0` at `min` and `1.0` at `max`.
pub fn value_to_percent(value: f64, min: f64, max: f64) -> f64 {
    if max <= min {
        return 0.0;
    }
    ((value - min) / (max - min)).clamp(0.0, 1.0)
}

/// Maps a normalized `[0.0, 1.0]` track fraction to a value in `[min, max]`.
pub fn fraction_to_value(fraction: f64, min: f64, max: f64) -> f64 {
    if max <= min {
        return min;
    }
    min + fraction.clamp(0.0, 1.0) * (max - min)
}

/// The thumb index closest to `target`. Ties go to the lower-indexed thumb.
pub fn nearest_thumb(values: &[f64], target: f64) -> usize {
    let mut best = 0;
    let mut best_distance = f64::MAX;

    for (index, thumb) in values.iter().enumerate() {
        let distance = (thumb - target).abs();
        if distance < best_distance {
            best_distance = distance;
            best = index;
        }
    }

    best
}

/// Headless slider state supporting single and multi-thumb ranges with minimum thumb separation.
#[derive(Clone, Debug, PartialEq)]
pub struct SliderState {
    pub values: Vec<f64>,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub min_steps_between_thumbs: usize,
}

impl SliderState {
    pub fn new(values: Vec<f64>, min: f64, max: f64, step: f64) -> Self {
        let values = values
            .into_iter()
            .map(|value| clamp_and_step(value, min, max, step))
            .collect();
        Self {
            values,
            min,
            max,
            step,
            min_steps_between_thumbs: 0,
        }
    }

    pub fn with_min_steps_between_thumbs(mut self, steps: usize) -> Self {
        self.min_steps_between_thumbs = steps;
        self
    }

    /// The `(lower, upper)` bounds thumb `index` may move within, respecting neighbours and
    /// `min_steps_between_thumbs`.
    pub fn bounds(&self, index: usize) -> (f64, f64) {
        let min_gap = if self.step > 0.0 {
            self.min_steps_between_thumbs as f64 * self.step
        } else {
            0.0
        };
        let lower = if index == 0 {
            self.min
        } else {
            self.values
                .get(index - 1)
                .map(|v| (v + min_gap).min(self.max))
                .unwrap_or(self.min)
        };
        let upper = if index + 1 >= self.values.len() {
            self.max
        } else {
            self.values
                .get(index + 1)
                .map(|v| (v - min_gap).max(self.min))
                .unwrap_or(self.max)
        };
        (lower, upper.max(lower))
    }

    /// Sets thumb `index` to `value`, snapped to `step` and clamped to its neighbour bounds.
    /// Returns `true` if the thumb value changed.
    pub fn set_at(&mut self, index: usize, value: f64) -> bool {
        if index >= self.values.len() {
            return false;
        }
        let (lower, upper) = self.bounds(index);
        let next = clamp_and_step(value, self.min, self.max, self.step).clamp(lower, upper);
        if self.values[index] != next {
            self.values[index] = next;
            true
        } else {
            false
        }
    }

    /// Nudges thumb `index` by `steps` multiples of `step`. Returns `true` if changed.
    pub fn step_by(&mut self, index: usize, steps: f64) -> bool {
        let Some(&current) = self.values.get(index) else {
            return false;
        };
        let step = if self.step > 0.0 { self.step } else { 1.0 };
        self.set_at(index, current + steps * step)
    }

    /// Sets thumb `index` from a normalized `[0.0, 1.0]` track fraction. Returns `true` if changed.
    pub fn set_fraction(&mut self, index: usize, fraction: f64) -> bool {
        let value = fraction_to_value(fraction, self.min, self.max);
        self.set_at(index, value)
    }

    /// Normalized `[0.0, 1.0]` position of each thumb.
    pub fn percents(&self) -> Vec<f64> {
        self.values
            .iter()
            .map(|&value| value_to_percent(value, self.min, self.max))
            .collect()
    }

    /// `(start, length)` in `[0.0, 1.0]` for the filled track segment.
    ///
    /// For one thumb, starts at `0.0` and ends at the thumb. For multiple thumbs, spans between
    /// the lowest and highest thumb.
    pub fn range_percent(&self) -> (f64, f64) {
        if self.values.is_empty() {
            return (0.0, 0.0);
        }
        let lowest = self.values.iter().copied().fold(f64::MAX, f64::min);
        let highest = self.values.iter().copied().fold(f64::MIN, f64::max);
        let from = if self.values.len() > 1 {
            lowest
        } else {
            self.min
        };
        let start = value_to_percent(from, self.min, self.max);
        let end = value_to_percent(highest, self.min, self.max);
        (start, (end - start).max(0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimals_of_counts_fractional_digits() {
        assert_eq!(decimals_of(1.0), 0);
        assert_eq!(decimals_of(0.5), 1);
        assert_eq!(decimals_of(0.25), 2);
        assert_eq!(decimals_of(0.125), 3);
    }

    #[test]
    fn clamp_and_step_snaps_without_float_drift() {
        assert_eq!(clamp_and_step(0.30000004, 0.0, 1.0, 0.1), 0.3);
        assert_eq!(clamp_and_step(-5.0, 0.0, 100.0, 5.0), 0.0);
        assert_eq!(clamp_and_step(150.0, 0.0, 100.0, 5.0), 100.0);
        assert_eq!(clamp_and_step(42.0, 0.0, 100.0, 0.0), 42.0);
    }

    #[test]
    fn value_and_fraction_conversions_round_trip() {
        assert_eq!(value_to_percent(50.0, 0.0, 100.0), 0.5);
        assert_eq!(value_to_percent(0.0, 10.0, 10.0), 0.0);
        assert_eq!(fraction_to_value(0.25, 20.0, 60.0), 30.0);
        assert_eq!(fraction_to_value(2.0, 0.0, 100.0), 100.0);
    }

    #[test]
    fn nearest_thumb_picks_closest_and_breaks_ties_low() {
        assert_eq!(nearest_thumb(&[20.0, 80.0], 30.0), 0);
        assert_eq!(nearest_thumb(&[20.0, 80.0], 70.0), 1);
        assert_eq!(nearest_thumb(&[20.0, 80.0], 50.0), 0);
    }

    #[test]
    fn slider_state_enforces_bounds_and_min_steps_between_thumbs() {
        let mut slider =
            SliderState::new(vec![20.0, 80.0], 0.0, 100.0, 5.0).with_min_steps_between_thumbs(2);

        // Min gap is 2 * 5 = 10, so thumb 0 cannot pass 70 and thumb 1 cannot go below 30.
        assert!(slider.set_at(0, 75.0));
        assert_eq!(slider.values, vec![70.0, 80.0]);
        assert!(!slider.set_at(0, 75.0));

        assert!(slider.step_by(0, -4.0));
        assert_eq!(slider.values, vec![50.0, 80.0]);

        assert!(slider.set_fraction(1, 0.0));
        assert_eq!(slider.values, vec![50.0, 60.0]);
        assert_eq!(slider.percents(), vec![0.5, 0.6]);
        let (start, len) = slider.range_percent();
        assert!((start - 0.5).abs() < 1e-9 && (len - 0.1).abs() < 1e-9);
    }

    #[test]
    fn single_thumb_range_percent_starts_at_zero() {
        let mut slider = SliderState::new(vec![25.0], 0.0, 100.0, 1.0);
        assert_eq!(slider.range_percent(), (0.0, 0.25));
        slider.set_fraction(0, 0.75);
        assert_eq!(slider.range_percent(), (0.0, 0.75));
        assert!(!slider.set_at(5, 10.0));
    }
}
