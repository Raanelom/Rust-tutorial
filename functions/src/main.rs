fn main() {
    println!("Hello, world!");
    another_function(5, -1000);
    some_expression();
    let x = five();
    println!("Five={}", x);
    println!("Plus 1 = {}", plus_one(x));
}

fn another_function(x: u32, y: i64) {
    println!("I have some variable x={} and another one y={}", x, y);
}

fn some_expression() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}