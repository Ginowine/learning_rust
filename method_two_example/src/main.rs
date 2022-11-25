fn main() {

    let rect1 = Rectangle{
        width: 30,
        heigh: 50,
    };

    let rect2 = Rectangle{
        width: 10,
        heigh: 40,
    };

    let rect3 = Rectangle{
        width: 60,
        heigh: 45,
    };

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

struct Rectangle {
    width: u32,
    heigh: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


