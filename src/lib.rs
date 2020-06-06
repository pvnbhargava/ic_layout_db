#[derive(Debug)]
struct Rectangle {
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

impl Rectangle {
    fn new() -> Self {
        return Rectangle {
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        };
    }
}

fn main() {
    println!("Hello World!");
    let new_rect = Rectangle::new();
    println!("{:?}", new_rect);
}

#[cfg(test)]
mod tests {
    use crate::main;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        main();
    }
}
