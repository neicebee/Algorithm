use std::io;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)
        .unwrap();
    buf.pop();
    buf
}

fn main() {
    let x = input().parse::<i32>().unwrap();
    let n = input().parse::<i32>().unwrap();
    let mut s = 0;
    (0..n).for_each(
        |_| {
            let mut tmp = 1;
            input().split(' ').for_each(
                |y| {
                    tmp*=y.parse::<i32>().unwrap()
                }
            );
            s+=tmp;
        }
    );
    println!("{}", if x==s { "Yes" } else { "No" });
}