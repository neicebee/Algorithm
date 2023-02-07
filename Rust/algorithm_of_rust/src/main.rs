use std::io;

fn main() {
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers)
        .expect("No input");
    let split_nums: Vec<f64> = numbers.split(' ')
        .map(
            |x| x.trim().parse().expect("error")
        ).collect();
    println!("{:?}", split_nums);
}