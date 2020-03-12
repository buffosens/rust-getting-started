struct User
{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User
{
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

pub fn process()
{

    // to be able to change a single entry, the whole instance but be mutable!
    let mut volker = User {
        username: String::from("volker"),
        email: String::from("this@is.mail"),
        sign_in_count: 12,
        active: false,
    };

    volker.email = String::from("new@my.mail");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: volker.active,
        sign_in_count: volker.sign_in_count,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        .. volker
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit-like structs : Unit-like structs can be useful in situations in which you need to
    // implement a trait on some type but don’t have any data that you want to store in the type itself.

    struct UserFail {
        username: &str,
        // wont work
        email: &str,
        // wont work
        sign_in_count: u64,
        active: bool,
    }
}