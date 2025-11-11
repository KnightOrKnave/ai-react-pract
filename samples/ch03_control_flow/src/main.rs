fn main() {
    let temperature = 27;
    let mood = if temperature >= 25 { "アイスコーヒー" } else { "ホットコーヒー" };
    println!("Weather helper recommends: {mood}");

    let score = 82;
    let grade = match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    };
    println!("Grade for score {score}: {grade}");

    let mut attempts = 0;
    let first_success = loop {
        attempts += 1;
        if attempts == 3 {
            break attempts;
        }
    };
    println!("Loop returned value {first_success}");

    let tasks = ["plan", "code", "test", "deploy"];
    for (index, task) in tasks.iter().enumerate() {
        println!("Task #{index}: {task}");
    }

    let mut stack = vec!["match", "if", "loop"];
    while let Some(top) = stack.pop() {
        println!("popped control keyword: {top}");
    }
}
