pub fn process()
{
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let mut spaces = "   ";
    let spaces = spaces.len();
    /*
    let mut spaces = "   ";
    spaces = spaces.len();
    */
    // Error:  not allowed to mutate a variable’s type
}