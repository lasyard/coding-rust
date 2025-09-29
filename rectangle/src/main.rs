#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle::new(30, 50);
    dbg!(&rect);
    println!("The area of the rectangle is {}.", rect.area());
    let square = Rectangle::square(40);
    println!("The area of the square is {}.", square.area());
    println!("Can rect hold square? {}", rect.can_hold(&square));
}
