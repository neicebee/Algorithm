use std::io;

fn main() {
    let t: Vec<_> = (0..2)
        .map(
            |_| {
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                buf.trim().parse::<i32>().unwrap() > 0
            }
        ).collect();
    let q = match t[..] {
        [true,true] => 1,
        [false,true] => 2,
        [false,false] => 3,
        _ => 4,
    };
    println!("{q}");
}