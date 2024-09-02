struct User {
    username:String,
    email:String,
    sign_in_count: u64,
    active: bool,
}


fn main() {
    let mut user1 = User{
        username: String::from("Pyrisous"),
        email: String::from("brunomfiletti@gmail.com"),
        sign_in_count: 1,
        active: true,
    }

    let name = user1.username;
    user1.email = String::from("swengh@outlook.com");

    let user2 = User{
        email: String::from("another@example.com"),
        ..user1
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
