enum Message {
    Quit,
    Move { x: i32, y: u32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
    let m2 = Message::Move { x: 3, y: 5 };
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
