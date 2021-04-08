struct User {
    username: String,
    email: String,
    is_male: bool,
}

fn main() {
    let some_user = User {
        username: String::from("Henk"),
        email: String::from("henk@hotmail.com"),
        is_male: false
    };

    println!("{}", some_user.username);

    println!("Hello, world!");
}
