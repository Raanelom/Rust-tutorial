use rand::Rng;

fn main() {
    println!("Hello, world!");
    another_function(5, -1000);
    some_expression();
    let x = five();
    println!("Five={}", x);
    println!("Plus 1 = {}", plus_one(x));
    let secret_number = rand::thread_rng().gen_range(1, 11);
    println!("greater than five? {}", gt_five(secret_number));
    println!("single or duplicate: {}", multiply_if_gt_five(secret_number));
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

fn gt_five(some_no: i32) -> bool {
    if some_no > 5 {
        true
    } else {
        false
    }
}

fn multiply_if_gt_five(some_no: i32) -> i32 {
    let new_no = if some_no > 5 { some_no*some_no } else { some_no };
    new_no
}