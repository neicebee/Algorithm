use std::io;

fn result() -> i32 {
    let mut buf = String::new();
    let mut r = 0;
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    buf.split(' ')
        .for_each(|x| r+=x.parse::<i32>().unwrap());
    r
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let t = buf.parse::<i32>().unwrap();
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..t {
        v.push(result());
    }
    v.iter()
        .for_each(|x| println!("{x}"));
}