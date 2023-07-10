// use std::io::{BufReader, BufRead};

// fn main() {
//     let f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf: Vec<u32> = BufReader::new(f).lines()
//         .map(|x| x.unwrap().parse::<u32>().unwrap()%42).collect();
//     buf.sort();
//     buf.dedup();
//     println!("{}", buf.len());
// }

use std::io::{stdin, BufReader, BufRead};

fn main() {
    let mut buf: Vec<u32> = BufReader::new(stdin()).lines()
        .map(|x| x.unwrap().parse::<u32>().unwrap()%42).collect();
    buf.sort();
    buf.dedup();
    println!("{}", buf.len());
}