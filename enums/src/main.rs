enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Eurocent,
    Euro(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Eurocent => 1,
        Coin::Euro(state) => {
            println!("State euro from {:?}!", state);
            100
        }
    }
}

fn main() {
    enum IpAddr {
        V4(String),
        V6(String)
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_nullable_number = Some(5);
    let some_absent_number: Option<i32> = None;

    value_in_cents(Coin::Euro(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

