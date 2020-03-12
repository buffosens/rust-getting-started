#[derive(Debug)] // <-- use this for enabling debug output via println
struct Rectangle
{
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32
{
    rectangle.width * rectangle.height
}

pub fn process()
{
    let rect1 = Rectangle { width: 30, height: 50 };

    // println!("rect1 is {}", rect1); // fails: output format display not defined
    // println!("rect1 is {:?}", rect1); // fails: output format debug not defined
    println!("rect1 is {:#?}", rect1); // works with #[derive(Debug)] added to struct declaration

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}
