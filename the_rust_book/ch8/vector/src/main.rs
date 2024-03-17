enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new(); // Needs explicit type
    let v = vec![1, 2, 3]; // Infers type

    let mut v = Vec::new(); // Infers type by pushing values
    v.push(4);
    v.push(5);
    v.push(6);

    let third: &i32 = &v[2];
    println!("The thrid element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let v = vec![100, 32, 18];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![16, 32, 64];
    for i in &mut v {
        *i *= 2;
    }
    for i in &v {
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
