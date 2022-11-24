fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    // let width1 = 30;
    // let heigh1 = 50;

    //println!("The area of the rectangle is {} square pixels.", area(width1, heigh1));

    //let rect1 = (30, 50);

    // this println calls the tuple area function
    //println!("The area of the rectangle is {} square pixels.", area(rect1));

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

// this function calculates the area of a rectangle using primitive variables 
// fn area(width: u32, height:u32) -> u32 {
//     width * height
// }

// this function calculates area of a rectangle using a tuple 
// fn area(dimention: (u32, u32)) -> u32 {
//     dimention.0 * dimention.1
// }

fn area(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}
struct Rectangle {
    width: u32,
    height: u32,
}




