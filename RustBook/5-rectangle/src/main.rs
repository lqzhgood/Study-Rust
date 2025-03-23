#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let react1 = Rectangle::new(10, 20);
    let react2 = Rectangle::new(60, 50);

    let area = react1.area();
    let res = react2.can_hold(&react1);

    println!("area {area}");
    println!("res {res}");
    println!("react1 {:?}", react1);
    println!("react1 {:#?}", react1);

    dbg!(&react1);
}
