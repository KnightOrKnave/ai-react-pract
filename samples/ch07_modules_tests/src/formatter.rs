pub fn format_scores(rows: &[(String, f64)]) -> Vec<String> {
    rows.iter()
        .map(|(name, score)| format!("{name:<8} -> {:>5.1}%", score * 100.0))
        .collect()
}
