struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64
}

fn main() {
    let user1 = User {
        active: true,
        name: "JM KIM".to_string(),
        email: "jm@dunamu.com".to_string(),
        sign_in_count: 1
    };

    println!("{}, {}, {}, {}", user1.active, user1.name, user1.email, user1.sign_in_count)
}
