//! Pure SVG chart geometry, axis scaling and series types.

/// The drawing area, in viewBox units. Uniform scaling means these are the only numbers in the
/// file that are arbitrary.
pub const VIEW_WIDTH: f64 = 600.0;
pub const VIEW_HEIGHT: f64 = 300.0;

/// The five theme colours a series falls back to, in order.
pub const SERIES_COLORS: [&str; 5] = [
    "var(--chart-1)",
    "var(--chart-2)",
    "var(--chart-3)",
    "var(--chart-4)",
    "var(--chart-5)",
];

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChartKind {
    #[default]
    Line,
    /// A line with the space below it filled.
    Area,
    /// One bar per series per category, grouped.
    Bar,
}

/// One line, area or set of bars. `values` lines up with the chart's labels by index.
#[derive(Clone, PartialEq, Debug)]
pub struct ChartSeries {
    pub key: String,
    pub label: String,
    /// Any CSS colour. Empty takes the theme colour for this series' position.
    pub color: String,
    pub values: Vec<f64>,
}

impl ChartSeries {
    pub fn new(key: impl Into<String>, label: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            color: String::new(),
            values,
        }
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    /// The colour to draw with: the one given, or this position's theme colour.
    pub fn resolved_color(&self, index: usize) -> String {
        if self.color.is_empty() {
            SERIES_COLORS[index % SERIES_COLORS.len()].to_string()
        } else {
            self.color.clone()
        }
    }
}

/// Rounds a maximum up to something a person would choose for the top of an axis: 1, 2, 2.5 or 5
/// times a power of ten. An axis topped at exactly the data's maximum has no headroom and its
/// labels are unreadable numbers.
pub fn nice_ceiling(max: f64) -> f64 {
    if max <= 0.0 {
        return 1.0;
    }

    let magnitude = 10f64.powf(max.log10().floor());
    for step in [1.0, 2.0, 2.5, 5.0, 10.0] {
        let candidate = step * magnitude;
        if candidate >= max {
            return candidate;
        }
    }

    max
}

/// Trims a tick's decimals without turning 1500 into "2k" — an axis that rounds its own labels
/// lies about the grid line beside it.
pub fn format_tick(value: f64) -> String {
    if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    }
}

/// Pure viewBox-space chart geometry and path layout.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ChartLayout {
    pub width: f64,
    pub height: f64,
    pub pad_left: f64,
    pub pad_right: f64,
    pub pad_top: f64,
    pub pad_bottom: f64,
    /// How many grid intervals the value axis is divided into.
    pub ticks: usize,
}

impl Default for ChartLayout {
    fn default() -> Self {
        Self {
            width: VIEW_WIDTH,
            height: VIEW_HEIGHT,
            pad_left: 44.0,
            pad_right: 12.0,
            pad_top: 12.0,
            pad_bottom: 28.0,
            ticks: 4,
        }
    }
}

impl ChartLayout {
    pub fn plot_width(&self) -> f64 {
        self.width - self.pad_left - self.pad_right
    }

    pub fn plot_height(&self) -> f64 {
        self.height - self.pad_top - self.pad_bottom
    }

    pub fn bottom(&self) -> f64 {
        self.height - self.pad_bottom
    }

    /// The largest value across every series, rounded up to a readable axis top.
    pub fn max(&self, series: &[ChartSeries]) -> f64 {
        let max = series
            .iter()
            .flat_map(|series| series.values.iter().copied())
            .fold(0.0f64, f64::max);

        nice_ceiling(max)
    }

    /// The value each grid line stands for, from the top down.
    pub fn ticks_for(&self, series: &[ChartSeries]) -> Vec<f64> {
        if self.ticks == 0 {
            return Vec::new();
        }
        let max = self.max(series);
        (0..=self.ticks)
            .map(|tick| max * (self.ticks - tick) as f64 / self.ticks as f64)
            .collect()
    }

    /// Where a category sits along the bottom. Points sit on the edges for a line and in the
    /// middle of a band for bars, which is the difference between "a value at this moment" and "a
    /// value for this period".
    pub fn x(&self, index: usize, count: usize, kind: ChartKind) -> f64 {
        match kind {
            ChartKind::Bar => {
                let band = self.plot_width() / count.max(1) as f64;
                self.pad_left + band * index as f64 + band / 2.0
            }
            _ if count > 1 => self.pad_left + self.plot_width() * index as f64 / (count - 1) as f64,
            _ => self.pad_left + self.plot_width() / 2.0,
        }
    }

    pub fn y(&self, value: f64, max: f64) -> f64 {
        if max <= 0.0 {
            return self.bottom();
        }
        self.pad_top + self.plot_height() * (1.0 - (value / max).clamp(0.0, 1.0))
    }

    /// Where a category sits across the plot, as a percentage — what an HTML tooltip needs, since
    /// it is positioned in the container rather than in the SVG.
    pub fn x_percent(&self, index: usize, count: usize, kind: ChartKind) -> f64 {
        if self.width <= 0.0 {
            return 0.0;
        }
        self.x(index, count, kind) / self.width * 100.0
    }

    /// `M … L …` through a series' points.
    pub fn line_path(
        &self,
        series: &ChartSeries,
        count: usize,
        max: f64,
        kind: ChartKind,
    ) -> String {
        series
            .values
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let command = if index == 0 { 'M' } else { 'L' };
                format!(
                    "{command}{:.2} {:.2}",
                    self.x(index, count, kind),
                    self.y(*value, max)
                )
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// The same path, closed along the bottom.
    pub fn area_path(
        &self,
        series: &ChartSeries,
        count: usize,
        max: f64,
        kind: ChartKind,
    ) -> String {
        if series.values.is_empty() {
            return String::new();
        }

        let last = series.values.len() - 1;
        format!(
            "{} L{:.2} {:.2} L{:.2} {:.2} Z",
            self.line_path(series, count, max, kind),
            self.x(last, count, kind),
            self.bottom(),
            self.x(0, count, kind),
            self.bottom()
        )
    }

    /// One bar: `(x, y, width, height)`. Bars are grouped inside their category's band, with a
    /// gap between bands so the grouping reads.
    pub fn bar(
        &self,
        index: usize,
        series_index: usize,
        value: f64,
        count: usize,
        series_count: usize,
        max: f64,
    ) -> (f64, f64, f64, f64) {
        let count = count.max(1);
        let series_count = series_count.max(1);

        let band = self.plot_width() / count as f64;
        let group = band * 0.72;
        let width = group / series_count as f64;
        let left = self.pad_left + band * index as f64 + (band - group) / 2.0;

        let y = self.y(value, max);

        (
            left + width * series_index as f64,
            y,
            width.max(1.0),
            (self.bottom() - y).max(0.0),
        )
    }

    /// The category nearest a pointer's x position in viewBox coordinates.
    pub fn index_at(&self, x_in_viewbox: f64, count: usize, kind: ChartKind) -> Option<usize> {
        if count == 0 {
            return None;
        }

        let index = match kind {
            ChartKind::Bar => {
                let band = self.plot_width() / count as f64;
                ((x_in_viewbox - self.pad_left) / band).floor()
            }
            _ if count > 1 => {
                let step = self.plot_width() / (count - 1) as f64;
                ((x_in_viewbox - self.pad_left) / step).round()
            }
            _ => 0.0,
        };

        Some((index.max(0.0) as usize).min(count - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_ceiling_rounds_to_readable_axis_tops() {
        assert_eq!(nice_ceiling(0.0), 1.0);
        assert_eq!(nice_ceiling(-5.0), 1.0);
        assert_eq!(nice_ceiling(1.0), 1.0);
        assert_eq!(nice_ceiling(12.0), 20.0);
        assert_eq!(nice_ceiling(23.0), 25.0);
        assert_eq!(nice_ceiling(42.0), 50.0);
        assert_eq!(nice_ceiling(75.0), 100.0);
        assert_eq!(nice_ceiling(186.0), 200.0);
    }

    #[test]
    fn ticks_divide_axis_from_top_down_and_format_cleanly() {
        let layout = ChartLayout::default();
        let series = vec![ChartSeries::new("a", "A", vec![10.0, 90.0])];
        assert_eq!(layout.max(&series), 100.0);
        assert_eq!(
            layout.ticks_for(&series),
            vec![100.0, 75.0, 50.0, 25.0, 0.0]
        );
        assert_eq!(format_tick(1500.0), "1500");
        assert_eq!(format_tick(12.5), "12.5");
    }

    #[test]
    fn line_and_area_paths_use_expected_coordinates() {
        let layout = ChartLayout::default();
        let series = ChartSeries::new("s", "S", vec![0.0, 50.0, 100.0]);
        let max = layout.max(std::slice::from_ref(&series));
        assert_eq!(max, 100.0);

        let line = layout.line_path(&series, 3, max, ChartKind::Line);
        assert_eq!(line, "M44.00 272.00 L316.00 142.00 L588.00 12.00");

        let area = layout.area_path(&series, 3, max, ChartKind::Area);
        assert_eq!(
            area,
            "M44.00 272.00 L316.00 142.00 L588.00 12.00 L588.00 272.00 L44.00 272.00 Z"
        );
        assert_eq!(
            layout.area_path(
                &ChartSeries::new("empty", "Empty", vec![]),
                0,
                100.0,
                ChartKind::Area
            ),
            ""
        );
    }

    #[test]
    fn bars_are_grouped_inside_category_bands() {
        let layout = ChartLayout::default();
        let (x0, y0, w0, h0) = layout.bar(0, 0, 50.0, 2, 2, 100.0);
        let (x1, y1, w1, h1) = layout.bar(0, 1, 100.0, 2, 2, 100.0);
        assert_eq!(w0, w1);
        assert!(x1 > x0);
        assert!((x1 - x0 - w0).abs() < 1e-9);
        assert_eq!(y1, layout.pad_top);
        assert_eq!(h1, layout.plot_height());
        assert!((y0 - (layout.pad_top + layout.plot_height() / 2.0)).abs() < 1e-9);
        assert!((h0 - layout.plot_height() / 2.0).abs() < 1e-9);
    }

    #[test]
    fn index_at_clamps_and_distinguishes_line_and_bar() {
        let layout = ChartLayout::default();
        assert_eq!(layout.index_at(100.0, 0, ChartKind::Line), None);
        assert_eq!(layout.index_at(0.0, 1, ChartKind::Line), Some(0));
        assert_eq!(layout.index_at(0.0, 3, ChartKind::Line), Some(0));
        assert_eq!(layout.index_at(316.0, 3, ChartKind::Line), Some(1));
        assert_eq!(layout.index_at(700.0, 3, ChartKind::Line), Some(2));

        let band = layout.plot_width() / 3.0;
        assert_eq!(
            layout.index_at(layout.pad_left + band * 0.9, 3, ChartKind::Bar),
            Some(0)
        );
        assert_eq!(
            layout.index_at(layout.pad_left + band * 1.1, 3, ChartKind::Bar),
            Some(1)
        );
        assert_eq!(
            layout.index_at(layout.pad_left + band * 2.5, 3, ChartKind::Bar),
            Some(2)
        );
    }
}
