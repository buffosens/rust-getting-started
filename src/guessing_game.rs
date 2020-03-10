use std::cmp::Ordering;
use rand::Rng;

pub fn process()
{
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // let -> new variable (let foo = bar), immutable by default
    // mut -> mutable
    let mut guess = String::new();

    // note the &mut as mutable reference
    std::io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // trim method on a String instance will eliminate any whitespace at the beginning and end
    let guess: u32 = guess.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}