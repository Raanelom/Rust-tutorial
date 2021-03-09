
fn main() {
    let x = 5;
    // Shadowing x
    let x = x + 1;
    // And again
    let x = x * 2;
    println!("The value of x is: {}", x);

    perform_math_operations();

    let a = [3; 5];
    println!("{}", a[1])
}


fn perform_math_operations() {
    // addition
    let sum = 5 + 10;
    println!("sum {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("remainder {}", remainder);
}