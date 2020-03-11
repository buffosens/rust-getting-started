fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

pub fn process()
{
    println!("This is main!");

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("Hello: {}, world: {}", hello, world);

    let s = String::from("hello");
    let _slice = &s[0..2]; // the same
    let slice = &s[..2];
    println!("slice: {}", slice);

    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[3..len]; // the same
    let slice = &s[3..];
    println!("slice: {}", slice);

    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[0..len];
    let slice = &s[..];
    println!("slice: {}", slice);

    let my_string = String::from("hello world");
    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    println!("word: {}", word);
    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("word: {}", word);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("word: {}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice: {:?}", slice);
}