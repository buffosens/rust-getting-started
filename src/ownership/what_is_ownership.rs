/*
 Some languages have garbage collection that constantly looks for no longer used memory as the
 program runs; in other languages, the programmer must explicitly allocate and free the memory.
 Rust uses a third approach: memory is managed through a system of ownership with a set of rules
 that the compiler checks at compile time. None of the ownership features slow down your program
 it’s running.
 */

/*
 Ownership Rules
    * Each value in Rust has a variable that’s called its owner.
    * There can only be one owner at a time.
    * When the owner goes out of scope, the value will be dropped.
 */


fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

pub fn process()
{
    {                            // s is not valid here, it’s not yet declared
        let s = "hello";    // s is valid from this point forward
        // do stuff with s
    }

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    {
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    }  // this scope is now over, and s is no longer valid

    {
        let s1 = gives_ownership();         // gives_ownership moves its return value into s1
        let s2 = String::from("hello");     // s2 comes into scope
        let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also
        // moves its return value into s3
    }
    // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
    // moved, so nothing happens. s1 goes out of scope and is dropped.
}