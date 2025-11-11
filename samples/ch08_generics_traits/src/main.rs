use std::ops::Add;

#[derive(Debug)]
struct Pair<T> {
    left: T,
    right: T,
}

impl<T> Pair<T> {
    fn new(left: T, right: T) -> Self {
        Self { left, right }
    }

    fn swap(self) -> Self {
        Self {
            left: self.right,
            right: self.left,
        }
    }
}

impl<T> Pair<T>
where
    T: Add<Output = T> + Copy,
{
    fn sum(&self) -> T {
        self.left + self.right
    }
}

trait Drawable {
    fn draw(&self) -> String;
}

struct Button {
    label: String,
}

struct Spacer;

impl Drawable for Button {
    fn draw(&self) -> String {
        format!("[ {} ]", self.label)
    }
}

impl Drawable for Spacer {
    fn draw(&self) -> String {
        "---".to_string()
    }
}

fn render_scene(components: &[Box<dyn Drawable>]) {
    for (index, component) in components.iter().enumerate() {
        println!("{index}: {}", component.draw());
    }
}

fn main() {
    let pair = Pair::new(3, 5);
    println!("pair: {:?}, sum: {}", pair, pair.sum());
    println!("swapped pair: {:?}", pair.swap());

    let float_pair = Pair::new(1.2_f64, 3.4_f64);
    println!("float pair sum: {}", float_pair.sum());

    let components: Vec<Box<dyn Drawable>> = vec![
        Box::new(Button {
            label: "Start".into(),
        }),
        Box::new(Spacer),
        Box::new(Button {
            label: "Stop".into(),
        }),
    ];
    render_scene(&components);
}
