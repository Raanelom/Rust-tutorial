fn main() {
    println!("Hello, world!");
    vector();
}

fn vector() {
    let mut v: Vec<i32> = Vec::new();
    let v_inferred = vec![1, 2, 3];
    v.push(5);
    v.push(8);

    let second: &i32 = &v[1];

    match v.get(1) {
        Some(third) => println!("The second element is {}", second),
        None => println!("There is no second element."),
    }

    // Throws an error
    //let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    // Iterating over vector:
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
