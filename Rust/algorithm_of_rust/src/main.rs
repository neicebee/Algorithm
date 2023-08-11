// use std::io::Read;

// fn main() {
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();
//     let (n, b) = {
//         let mut t = buf.trim().split(' ');
//         (t.next().unwrap().parse::<u32>().unwrap(), 
//         t.next().unwrap().parse::<u32>().unwrap())
//     };
//     println!("{}", convert(n, b));
// }

use std::io;

fn main() {
    println!("{}", 124%1000/100);
}