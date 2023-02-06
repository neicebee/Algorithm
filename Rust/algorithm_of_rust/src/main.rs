use std::io;

fn main() {
    let mut numbers = String::new();
    let split_nums: Vec<&str> = match io::stdin().read_line(&mut numbers) {
        Ok(_n) => numbers.split(' ').collect(),
        Err(_) => vec!["Error"],
    };
    let a: u32 = match split_nums[0].trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };
    let b: u32 = match split_nums[1].trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };
    println!("{}", a+b);
}