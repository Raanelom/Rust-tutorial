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

    let word_with_spaces = String::from("some word with spaces");
    let first_word = first_word(&word_with_spaces);
    
    println!("first word in \"{}\" is \"{}\"", word_with_spaces, first_word);
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
    pass_reference(&s2);
    // Can use s2 here
    println!("can still use {}", s2)
}

fn hand_over_ownership(s: String) {
    println!("{}", s);
}

fn keep_ownership(s: String) -> String {
    s
}

fn pass_reference(s: &String) -> usize {
    s.len()
}

/// Return the index of the first space
/// Return the word-length otherwise
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}