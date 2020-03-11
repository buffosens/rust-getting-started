fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s --> 's' will work
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

pub fn process()
{
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s; // wont work: cannot borrow `s` as mutable more than once at a time
    println!("{}, {}", r1, r2);

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);
}