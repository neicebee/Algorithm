// use std::fmt::Display;

// fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
//     where T: Display
// {
//     println!("Attention! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let msg1 = String::from("abcd");
//     let msg2 = String::from("xyz");

//     let result = longest_with_an_announcement(
//         msg1.as_str(), msg2.as_str(), "Everyone!!"
//     );

//     println!("result: {}", result);
// }

fn main() {
    let msg = "EOF";
    for b in msg.bytes() {
        println!("{b:x}");
    }
}