struct User {
    active: bool,
    name: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, name: String) -> User {
    User {
        active: true, name, email, sign_in_count: 1
    }
}

fn main() {
    let mut user1 = build_user("jm6534@gmail.com".to_string(), "KJM".to_string());
    let user2: User = User { name: "kjm".to_string(), ..user1 };

    user1.email = String::from("minn987@naver.com");

    println!("{}, {}, {}, {}", user1.active, user1.name, user1.email, user1.sign_in_count);
    println!("{}, {}, {}, {}", user2.active, user2.name, user2.email, user2.sign_in_count);
}
