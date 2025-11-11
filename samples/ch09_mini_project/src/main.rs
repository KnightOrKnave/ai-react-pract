use std::{
    env,
    error::Error,
    fmt,
    fs,
    io::{self, ErrorKind},
};

fn main() {
    if let Err(err) = run() {
        eprintln!("tasker: {err}");
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        print_usage();
        return Ok(());
    }

    let command = Command::parse(&args)?;
    let path = "tasks.txt";
    let mut tasks = TaskList::load(path)?;

    match command {
        Command::Add(title) => {
            tasks.add(title);
            tasks.save(path)?;
            println!("Task added. Total tasks: {}", tasks.len());
        }
        Command::Done(index) => {
            tasks.mark_done(index)?;
            tasks.save(path)?;
            println!("Task #{index} marked as done.");
        }
        Command::List => tasks.print(),
        Command::Clear => {
            tasks.clear();
            tasks.save(path)?;
            println!("Tasks cleared.");
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Usage:");
    println!("  cargo run -- add \"Write docs\"");
    println!("  cargo run -- done 0");
    println!("  cargo run -- list");
    println!("  cargo run -- clear");
}

#[derive(Debug)]
enum Command {
    Add(String),
    Done(usize),
    List,
    Clear,
}

impl Command {
    fn parse(args: &[String]) -> Result<Self, CommandError> {
        match args.get(0).map(String::as_str) {
            Some("add") => {
                let title = args[1..].join(" ");
                if title.trim().is_empty() {
                    Err(CommandError("task title is empty".into()))
                } else {
                    Ok(Command::Add(title))
                }
            }
            Some("done") => {
                let index = args
                    .get(1)
                    .ok_or_else(|| CommandError("missing task index".into()))?
                    .parse::<usize>()
                    .map_err(|_| CommandError("index must be a number".into()))?;
                Ok(Command::Done(index))
            }
            Some("list") => Ok(Command::List),
            Some("clear") => Ok(Command::Clear),
            Some(other) => Err(CommandError(format!("unknown command: {other}"))),
            None => Err(CommandError("no command supplied".into())),
        }
    }
}

#[derive(Debug)]
struct CommandError(String);

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CommandError {}

#[derive(Debug, Clone)]
struct Task {
    title: String,
    done: bool,
}

#[derive(Default)]
struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    fn load(path: &str) -> io::Result<Self> {
        match fs::read_to_string(path) {
            Ok(contents) => Ok(Self::from_string(&contents)),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(Self::default()),
            Err(err) => Err(err),
        }
    }

    fn from_string(data: &str) -> Self {
        let tasks = data
            .lines()
            .filter_map(|line| {
                if line.len() < 4 || !line.starts_with('[') {
                    return None;
                }
                let done = matches!(line.chars().nth(1), Some('x') | Some('X'));
                let title = line[4..].trim().to_string();
                if title.is_empty() {
                    return None;
                }
                Some(Task { title, done })
            })
            .collect();
        Self { tasks }
    }

    fn save(&self, path: &str) -> io::Result<()> {
        let mut buffer = String::new();
        for task in &self.tasks {
            let marker = if task.done { 'x' } else { ' ' };
            buffer.push('[');
            buffer.push(marker);
            buffer.push_str("] ");
            buffer.push_str(&task.title);
            buffer.push('\n');
        }
        fs::write(path, buffer)
    }

    fn len(&self) -> usize {
        self.tasks.len()
    }

    fn add(&mut self, title: String) {
        self.tasks.push(Task { title, done: false });
    }

    fn mark_done(&mut self, index: usize) -> Result<(), CommandError> {
        if let Some(task) = self.tasks.get_mut(index) {
            task.done = true;
            Ok(())
        } else {
            Err(CommandError(format!("no task at index {index}")))
        }
    }

    fn clear(&mut self) {
        self.tasks.clear();
    }

    fn print(&self) {
        if self.tasks.is_empty() {
            println!("No tasks yet. Use `add` to create one.");
            return;
        }

        for (index, task) in self.tasks.iter().enumerate() {
            let marker = if task.done { 'x' } else { ' ' };
            println!("#{index} [{marker}] {}", task.title);
        }
    }
}
