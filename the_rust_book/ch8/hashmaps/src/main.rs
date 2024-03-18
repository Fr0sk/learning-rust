use std::{collections::HashMap, io::stdin};

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // The HashMap owns them

    // Replace old value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);
    println!("{:?}", scores);

    // Ignore new value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    println!("\n\nExercises:\n");
    ex1(vec![3, 5, 67, 3, 23, 1, 1, 5, 6, 8, 2, 3, 1, 5, 1]);
    ex2(String::from("first"));
    ex2(String::from("apple"));
    ex3();
}

fn ex1(v: Vec<i32>) {
    let mut v = v.clone();
    v.sort();
    let median = v[v.len() / 2];

    let mut counts = HashMap::new();
    for n in v {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
    }

    let mut largest_count = 0;
    let mut mode = 0;
    for (k, v) in counts {
        if v > largest_count {
            largest_count = v;
            mode = k;
        }
    }

    println!("Ex1: Median: {median}, Mode: {mode}");
}

fn ex2(text: String) {
    let first_letter = text
        .to_lowercase()
        .chars()
        .next()
        .expect("String should not be empty");

    let mut result = text.clone();
    if vec!['a', 'e', 'i', 'o', 'u'].contains(&first_letter) {
        result.push_str("-hay");
    } else {
        result = format!("{}-{first_letter}ay", result[1..].to_string());
    }

    println!("Ex2: {}", result);
}

fn ex3() {
    let mut option = -1;
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    while option != 0 {
        println!("<<< Ex3 Menu >>>");
        println!("\t1 - Add employee");
        println!("\t2 - See employees");
        println!("\t0 - Quit");

        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");
        option = input.trim().parse().expect("Please type a number");

        if option == 1 {
            let mut name = String::new();
            let mut department = String::new();

            println!("Enter Employee Name: ");
            stdin().read_line(&mut name).expect("Failed to read line");

            println!("Enter Department Name: ");
            stdin()
                .read_line(&mut department)
                .expect("Failed to read line");

            department = department.trim().to_string();
            name = name.trim().to_string();

            let department_vec = map.entry(department).or_insert(Vec::new());
            department_vec.push(name);
        } else if option == 2 {
            println!("{:#?}", map)
        }
    }
}
