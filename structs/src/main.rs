fn main() {

    struct User{
        username: String,
        active: bool,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User{
        username: String::from("gino wine"),
        email: String::from("gino.wine@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("gino_wine@gmail.com");

    println!("{}, {}, {}, {}", user1.email, user1.username, user1.active, user1.sign_in_count);
}
