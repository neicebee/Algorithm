// use std::io::Read;

// fn main() {
//     const MAX: f64 = 100.0;
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();

//     let s: Vec<f64> = buf.lines().skip(1).next().unwrap()
//         .split(' ').map(|x| x.parse().unwrap()).collect();
//     let max = s.iter().fold(
//         -MAX, |max, n| n.max(max));
//     let mut r: Vec<f64> = Vec::new();
//     s.iter().for_each(|x| r.push(x/max*100.0));
//     println!("{}", r.iter().sum::<f64>()/r.len() as f64);
// }

use std::io::{self, Read};

fn main() {
    const MAX: f64 = 100.0;
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let s: Vec<f64> = buf.lines().skip(1).next().unwrap()
        .split(' ').map(|x| x.parse().unwrap()).collect();
    let max = s.iter().fold(
        -MAX, |max, n| n.max(max));
    let mut r: Vec<f64> = Vec::new();
    s.iter().for_each(|x| r.push(x/max*100.0));
    println!("{}", r.iter().sum::<f64>()/r.len() as f64);
}