pub fn process()
{
    // statically typed: must know the types of all variables at compile time

    // Wont work: consider giving `guess` a type
    // let guess = "42".parse().expect("Not a number!");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess: {}", guess);

    // scalar type: single value
    //      four primary scalar types: integers, floating-point numbers, booleans, and characters

    //      Integers:
    //          Decimal         98_222
    //          Hex             0xff
    //          Octal           0o77
    //          Binary          0b1111_0000
    //          Byte (u8 only)  b'A'

    // floating-point numbers:
    let _x = 2.0;   // f64
    let _y: f32 = 3.0;   // f32

    // booleans:
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // characters:
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Compound type: group multiple values
    //      two primitive compound types: tuples and arrays

    // tuple -  fixed length: once declared, they cannot grow or shrink in size:
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The values of x-y-z are: {}-{}-{}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    // array - every element of an array must have the same type, fixed length:
    // --> data allocated on the stack rather than the heap
    let a = [1, 2, 3, 4, 5];
    let _b: [i32; 5] = [1, 2, 3, 4, 5];
    let _c = [3; 5]; // --> let a = [3, 3, 3, 3, 3];

    let _first = a[0];
    let _second = a[1];
    // let index = 10;
    // let invalid = a[index]; // --> runtime error

    let _months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
}