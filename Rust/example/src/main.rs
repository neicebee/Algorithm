fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let msg1 = String::from("abcd");
    let result;
    {
        let msg2 = String::from("xyz");
        result = longest(msg1.as_str(), msg2.as_str());
    }
    println!("longest: {result}");
}