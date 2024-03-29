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
        other.width < self.width && other.height < self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 30,
    };

    println!(
        "The area of the rectangle {:#?} is {} square pixels.",
        rect1,
        rect1.area()
    );

    let scale = 2;
    let rect_debug = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect_debug);

    let rect1 = Rectangle {
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
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(23);
    println!("This is a square {:#?}", square);
}
