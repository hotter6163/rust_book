fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("comeusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    let user2 = build_user(String::from("sample@email.com"), String::from("sample"));

    println!("{}", user2.username);

    let user3 = User { ..user1 };

    user1.email = String::from("anotheremail@example.com");

    println!("{}", user1.email);

    println!("{}, {}", user2.email, user1.active);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
