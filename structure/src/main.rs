#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width >= other.width && self.height >= other.height)
            || (self.width >= self.height && self.height >= other.width)
    }

    fn square(size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        height: 10,
        width: 10,
    };
    let rect2 = Rectangle {
        height: 10,
        width: 40,
    };

    let sq = Rectangle.square(10);

    println!("The area of rectangle: {}", rect.area());

    dbg!(&rect);

    println!("Can rect2 hold rect?: {}", rect2.can_hold(&rect));
}

// fn calculate_area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
