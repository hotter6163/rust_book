#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("{}", rect1.height);
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}