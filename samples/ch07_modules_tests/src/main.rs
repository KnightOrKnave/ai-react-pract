use ch07_modules_tests::{format_scores, normalize_scores};

struct Student {
    name: &'static str,
    score: f64,
}

fn main() {
    let students = vec![
        Student {
            name: "Aoi",
            score: 72.0,
        },
        Student {
            name: "Jun",
            score: 88.0,
        },
        Student {
            name: "Mei",
            score: 94.0,
        },
    ];

    let raw_scores: Vec<f64> = students.iter().map(|student| student.score).collect();
    let normalized = normalize_scores(&raw_scores);

    let printable_rows: Vec<(String, f64)> = students
        .iter()
        .zip(normalized.iter())
        .map(|(student, score)| (student.name.to_string(), *score))
        .collect();

    for line in format_scores(&printable_rows) {
        println!("{line}");
    }
}
