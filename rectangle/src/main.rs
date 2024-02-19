#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn from(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn eunga() -> String {
        return String::from("응가");
    }
}

fn main() {
    let rect1 = Rectangle::from(4, 5);
    println!("{}", Rectangle::eunga());
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
