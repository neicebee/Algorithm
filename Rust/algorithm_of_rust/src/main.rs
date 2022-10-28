use std::io;

fn main(){

    let mut nums = String::new();
    io::stdin().read_line(&mut nums)
        .expect("Unable to read input!");
    
    let splitnums: Vec<&str> = nums.split(' ')
        .collect();

    let a = &splitnums[0];
    let b = &splitnums[1];

    let a: f64 = match a.trim().parse() {
        Ok(x) => x,
        Err(_) => 0.0,
    };

    let b: f64 = match b.trim().parse() {
        Ok(x) => x,
        Err(_) => 0.0,
    };

    println!("{}", a/b);
}