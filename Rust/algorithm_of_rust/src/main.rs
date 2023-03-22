use std::io;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop(); // trim()과 비슷한 작업을 함
    buf
}

fn split_num() -> Vec<i32> {
    let s = input();
    s.split(' ')
    .map(
        |x| x.parse().unwrap()
    ).collect()
}
fn main() {
    let dices = split_num();
    println!("{:?}", dices);
}