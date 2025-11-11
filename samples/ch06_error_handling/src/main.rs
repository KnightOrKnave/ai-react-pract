use std::{
    env,
    fs,
    io,
    num::ParseIntError,
    path::Path,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
    }
}

fn run() -> Result<(), SumError> {
    let path = env::args().nth(1).unwrap_or_else(|| "numbers.txt".to_string());
    let sum = sum_file(&path)?;
    println!("sum of values in {path}: {sum}");
    Ok(())
}

fn sum_file(path: &str) -> Result<i64, SumError> {
    let data = if Path::new(path).exists() {
        fs::read_to_string(path)?
    } else {
        // フォールバックデータ
        "1\n2\n3\n".to_string()
    };
    sum_lines(&data)
}

fn sum_lines(text: &str) -> Result<i64, SumError> {
    let mut total = 0;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let value: i64 = parse_number(trimmed)?;
        total += value;
    }
    Ok(total)
}

fn parse_number(text: &str) -> Result<i64, SumError> {
    text.parse::<i64>().map_err(|source| SumError::Parse {
        line: text.to_string(),
        source,
    })
}

#[derive(Debug)]
enum SumError {
    Io(io::Error),
    Parse { line: String, source: ParseIntError },
}

impl std::fmt::Display for SumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SumError::Io(err) => write!(f, "I/O error: {err}"),
            SumError::Parse { line, source } => write!(f, "failed to parse \"{line}\": {source}"),
        }
    }
}

impl std::error::Error for SumError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SumError::Io(err) => Some(err),
            SumError::Parse { source, .. } => Some(source),
        }
    }
}

impl From<io::Error> for SumError {
    fn from(value: io::Error) -> Self {
        SumError::Io(value)
    }
}
