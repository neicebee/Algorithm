// use std::io::Read;

// fn main() {
//     let mut l = [-1_i32; 26];
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();
//     let b: Vec<_> = buf.as_bytes().iter().map(|x| x-97).collect();
//     for (i, v) in b.iter().enumerate() {
//         if l[*v as usize] == -1 {
//             l[*v as usize] = i as i32;
//         }
//     }
//     println!("{:?}", l);
// }

// fn main() {
//     let mut l = [-1_i32; 26];
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();
//     let b: Vec<_> = buf.as_bytes().iter().map(|x| x-97).collect();
//     b.iter().enumerate().for_each(
//         |(i, v)| {
//             if l[*v as usize] == -1 {
//                 l[*v as usize] = i as i32;
//             }
//         }
//     );
//     println!("{:?}", l);
// }

use std::io::{self, *};

fn main() {
    let mut l = [-1_i32; 26];
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let b: Vec<_> = buf.as_bytes().iter().map(|x| x-97).collect();
    b.iter().enumerate().for_each(
        |(i, v)| {
            if l[*v as usize] == -1 {
                l[*v as usize] = i as i32;
            }
        }
    );
}