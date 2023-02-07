use std::io;

fn main() {
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers)
        .expect("No input");
    println!(
        "{:?}",
        numbers.trim().split(' ')
        .map(
            |x| x.parse().expect("error")
        ).fold(1, |a, b: i8| a*b)
    );
}