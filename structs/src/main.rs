struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("abc123"),
        email: String::from("a@gmail.com"),
        sign_in_count: 2
    };

    user1.email = String::from("b@gmail.com")
}
