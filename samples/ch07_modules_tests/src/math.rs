pub fn normalize_scores(scores: &[f64]) -> Vec<f64> {
    if scores.is_empty() {
        return Vec::new();
    }

    let min = scores
        .iter()
        .fold(f64::INFINITY, |acc, &value| acc.min(value));
    let max = scores
        .iter()
        .fold(f64::NEG_INFINITY, |acc, &value| acc.max(value));

    if (max - min).abs() < f64::EPSILON {
        return vec![1.0; scores.len()];
    }

    scores
        .iter()
        .map(|&score| (score - min) / (max - min))
        .collect()
}
