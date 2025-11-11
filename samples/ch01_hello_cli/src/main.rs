use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let name = args.next().unwrap_or_else(|| "Rustacean".to_string());
    let enthusiasm = args.next().unwrap_or_else(|| "normal".to_string());

    let greeting = match enthusiasm.as_str() {
        "loud" => format!("HELLO, {name}!"),
        "calm" => format!("Hello, {name}."),
        _ => format!("Hi, {name}! Ready to learn Rust?"),
    };

    println!("{greeting}");
    let build_mode = if cfg!(debug_assertions) { "debug" } else { "release" };
    println!("build mode: {build_mode}");
}
