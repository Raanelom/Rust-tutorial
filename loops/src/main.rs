fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // This returns the multiplied value
        }
        println!("still not there");
    };

    println!("The result is {}", result);
    mean_while();
    for_the_king();
    fancy_for();
}

fn mean_while() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_the_king() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("Arthur is {} years old", element);
    }
}

fn fancy_for() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}