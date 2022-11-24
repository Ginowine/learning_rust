fn main() {

    // declaring a user struct 
    struct User{
        username: String,
        active: bool,
        email: String,
        sign_in_count: u64,
    }

    // creating an instance of the user struct
    let mut user1 = User{
        username: String::from("gino wine"),
        email: String::from("gino.wine@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    // updating a field in a struct
    user1.email = String::from("gino_wine@gmail.com");

    // print struct values 
    println!("{}, {}, {}, {}", user1.email, user1.username, user1.active, user1.sign_in_count);
}

// this functions creates a new user and return it to the caller
fn build_user (email: String, username: String) -> User{
    User{
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// This function uses the field init shorthand to create new user since both parameter
// names are similar to structs names fields
fn build_user_init(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_from_another_instance() -> User{
    let user2 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("gino.wine.man@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };
}
