struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("phlucasfr"),
        email: String::from("phlucasfr@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
