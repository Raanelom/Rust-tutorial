fn main() {
    println!("Hello, world!");
    let s = "hello outer";
    {
        let s = "hello inner";
        println!("inner scope: {}", s);
    }
    println!("outer scope: {}", s);

    string_v2();
    test_string_scope();
}

fn string_v2() {
    let mut s = String::from("hello");
    s.push_str(" world");

    println!("{}", s);
}

fn test_string_scope() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world", s1);
    hand_over_ownership(s1);
    // Can't use s1 here anymore
}

fn hand_over_ownership(s: String) {
    println!("{}", s);
}

fn keep_ownership(s: String) -> String {
    s
}