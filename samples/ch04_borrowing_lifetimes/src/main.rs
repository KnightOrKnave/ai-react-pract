fn main() {
    let sentence = String::from("ownership rules keep us honest");
    let first = &sentence[..10];
    let second = &sentence[11..];
    let longer = longest(first, second);
    println!("longer slice is \"{longer}\"");

    let numbers = vec![3, 41, 11, 7];
    if let Some(max) = max_in_slice(&numbers) {
        println!("max value via immutable slice: {max}");
    }
    println!("original vector is still accessible: {:?}", numbers);
}

fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

fn max_in_slice(slice: &[i32]) -> Option<i32> {
    slice.iter().copied().max()
}
