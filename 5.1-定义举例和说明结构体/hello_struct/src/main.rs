struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let user1 = User {
        email: String::from("someone@example"),
        username: String::from("someoneusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
