fn another_function()
{
    println!("Another function.");
}

fn yet_another_function(x: i32)
{
    println!("The value of x is: {}", x);
}

fn and_yet_another_function(x: i32, y: i32)
{
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32
{
    5
}

fn plus_one(x: i32) -> i32
{
    x + 1
}

pub fn process()
{
    another_function();
    yet_another_function(7);
    and_yet_another_function(2,5);

    // ! Rust is an expression-based language
    let _x = 5;
    let y =
    // this block evaluates to '4'
    {
        let x = 3;
        x + 1 // note: no semicolon at the end, with semicolon --> statement, not an expression!
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}