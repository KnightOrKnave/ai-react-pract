fn main() {
    let greeting = String::from("Ownership feels strict");
    let emphasized = takes_ownership(greeting);
    println!("{emphasized}");

    let number = 42;
    print_copy(number);
    println!("number is still accessible after copy: {number}");

    let mut phrase = String::from("Borrowed data");
    describe_borrow(&phrase);
    enhance_phrase(&mut phrase);
    println!("after mutable borrow: {phrase}");
}

fn takes_ownership(mut text: String) -> String {
    text.push_str(" but keeps our data safe");
    text
}

fn print_copy(value: i32) {
    println!("Copy types can be reused immediately: {value}");
}

fn describe_borrow(text: &str) {
    println!("Borrowed \"{text}\" with length {}", text.chars().count());
}

fn enhance_phrase(text: &mut String) {
    text.push_str(" and lets multiple readers observe it");
}
