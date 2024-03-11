fn main() {
    let s = String::from("Hello Home World");
    let first = first_word(&s);
    let second = second_word(&s);
    println!("{}\n{}", first, second);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut start_index = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start_index == 0 {
                start_index = i;
            } else {
                return &s[start_index..i];
            }
        }
    }

    &s[start_index..]
}
