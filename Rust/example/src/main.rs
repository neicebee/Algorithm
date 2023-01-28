#[derive(Debug)]
struct User {
    username: String,
    pw: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

fn main() {
    let user1 = User{
        username: String::from("f1r3_r41n"),
        pw: String::from("a12345"),
        email: String::from("qkrghkql1@gmail.com"),
        sign_in_count: 3,
        active: true,
    };

    let user2 = User{
        username: String::from("gnuykob_"),
        pw: String::from("b12345"),
        email: String::from("leebk1124@gmail.com"),
        ..user1
    };

    println!("{:?}", user1);
    println!("{:?}", user2);
}