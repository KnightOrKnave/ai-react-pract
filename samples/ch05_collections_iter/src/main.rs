use std::collections::HashMap;

fn main() {
    let text = "Rust makes fearless fearless concurrency";
    let frequencies = word_frequencies(text);
    let mut ranking: Vec<_> = frequencies.iter().collect();
    ranking.sort_by(|a, b| b.1.cmp(a.1));

    println!("Top words:");
    for (word, count) in ranking.iter().take(3) {
        println!("  {word}: {count}");
    }

    let even_squares: Vec<i32> = (1..=10)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .collect();
    let sum: i32 = even_squares.iter().sum();
    println!("even squares: {even_squares:?}");
    println!("sum of even squares = {sum}");
}

fn word_frequencies(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let key = word.to_lowercase();
        *map.entry(key).or_insert(0) += 1;
    }
    map
}
