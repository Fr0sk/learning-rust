struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("some_username123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    user1.email = String::from("another_email@example.com");
    println!("{}", user1.email);

    let user2 = build_user(
        String::from("yetotheremail@example.com"),
        String::from("user2"),
    );
    println!("{}", user2.username);

    let user3 = User {
        email: String::from("user3@example.com"),
        ..user1
    };
    if user3.active {
        println!("{}", user3.sign_in_count);
    }
    // println!("{}", user1.username); Invalid
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
