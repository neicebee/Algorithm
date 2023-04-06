use std::io::{self, Read};

// fn result() -> Vec<i32> {
//     v
// }

fn main() {
    let mut buf = [0; 2];
    io::stdin().read(&mut buf).unwrap();
    let t: i32 = (buf[0]-b'0').into();
    let mut buf2 = String::new();
    io::stdin().read_line(&mut buf2).unwrap();
    buf2.pop();
    let v: Vec<i32> = buf2.split(' ')
        .map(
            |x| x.parse().unwrap()
        ).collect();

    println!("{t} : {:?}", v);
}