// use std::io::Read;

// fn main() {
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();
//     for l in buf.lines().skip(1) {
//         let mut iter = l.split(' ');
//         let (r, s) = (
//             iter.next().unwrap().parse::<i8>().unwrap(),
//             iter.next().unwrap().chars());
//         s.for_each(|x| (0..r).for_each(|_| print!("{x}")));
//         println!("");
//     }
// }

use std::io::{self, *};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    for l in buf.lines().skip(1) {
        let mut sp = l.split(' ');
        let (r, s) = (
            sp.next().unwrap().parse::<u8>().unwrap(),
            sp.next().unwrap().chars());
        s.for_each(|x| (0..r).for_each(|_| print!("{x}")));
        println!("");
    }
}