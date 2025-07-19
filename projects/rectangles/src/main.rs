#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let width: u32 = 30;
    let height: u32 = 50;

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let rect4 = Rectangle::square(100);

    // dbg!(&rect);

    println!("Can rect2 hold rect? {}", rect2.can_hold(&rect));
    println!("Can rect3 hold rect? {}", rect3.can_hold(&rect));
    println!("Can rect4 hold rect? {}", rect4.can_hold(&rect));

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area2((width, height))
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn area2(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
