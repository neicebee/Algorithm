fn main() {
    let s1 = give_ownership();
    let s2 = String::from("Rust!");
    println!("{}", s2);
    
    let s3 = take_and_give(s2);

    println!("{} {}", s1, s3);
}

fn give_ownership() -> String {
    let s = String::from("Rust!!");
    s
}

fn take_and_give(s: String) -> String {
    s
}