// use std::io::Read;

// fn main() {
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();
//     let mut v: Vec<Vec<u32>> = Vec::new();
//     for l in buf.lines() {
//         v.push(l.split_whitespace()
//             .map(|x| x.parse().unwrap())
//             .collect());
//     }
//     let mut max = v[0][0];
//     let (mut a, mut b) = (0, 0);
//     for i in 0..9 {
//         for j in 0..9 {
//             if v[i][j] > max {
//                 max = v[i][j];
//                 (a, b) = (i+1, j+1);
//             }
//         }
//     }
//     println!("{max}\n{a} {b}");
// }

use std::io::{self, *};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut v: Vec<Vec<i32>> = Vec::new();
    for l in buf.lines() {
        v.push(
            l.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect()
        );
    }
    let mut max = -1;
    let (mut a, mut b) = (0, 0);
    for i in 0..9 {
        for j in 0..9 {
            if v[i][j] > max {
                max = v[i][j];
                (a, b) = (i+1, j+1);
            }
        }
    }
    println!("{max}");
    println!("{a} {b}");
}