pub mod math;
pub mod formatter;

pub use formatter::format_scores;
pub use math::normalize_scores;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization_produces_unit_range() {
        let data = vec![50.0, 75.0, 100.0];
        let normalized = normalize_scores(&data);
        assert!(normalized.iter().all(|value| (0.0..=1.0).contains(value)));
    }

    #[test]
    fn formatter_aligns_output() {
        let rows = vec![(String::from("Rust"), 0.82)];
        let display = format_scores(&rows);
        assert!(display[0].contains("82"));
    }
}
