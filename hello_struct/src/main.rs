struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("test@henk.nl"), String::from("Henk"));

    let user3 = User {
        email: String::from("kindvan@henk.nl"),
        ..user2
    }; // Copy all the fields of user2 to user3, except email

    println!("user1 {}", user1.username);

    //println!("user2 {}", user2.username);

    println!("user3 {}", user3.username);

    println!("Hello, world!");

    use_some_colors();
}

fn use_some_colors() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{} {}", black.0, black.1);
}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}