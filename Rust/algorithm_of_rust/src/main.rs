// use std::io::Read;

// fn main() {
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();
//     let mut cnt = 0;
//     for w in buf.lines().skip(1) {
//         let mut c = [false; 26];
//         let t = w.trim().bytes();
//         c[(t.clone().nth(0).unwrap()-97) as usize] = true;
//         for i in 1..w.trim().len() {
//             if t.clone().nth(i).unwrap() == t.clone().nth(i-1).unwrap() {
//                 continue;
//             } else if t.clone().nth(i).unwrap() != t.clone().nth(i-1).unwrap() &&
//             c[(t.clone().nth(i).unwrap()-97) as usize] == true {
//                 cnt+=1;
//                 break;
//             } else {
//                 c[(t.clone().nth(i).unwrap()-97) as usize] = true;
//             }
//         }
//     }
//     println!("{}", buf.lines().skip(1).count()-cnt);
// }

use std::io::{self, *};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut cnt = 0;
    for w in buf.lines().skip(1) {
        let mut c = [false; 26];
        let t = w.trim().bytes();
        c[(t.clone().nth(0).unwrap()-97) as usize] = true;
        for i in 1..w.trim().len() {
            if t.clone().nth(i).unwrap() == t.clone().nth(i-1).unwrap() {
                continue;
            } else if t.clone().nth(i).unwrap() != t.clone().nth(i-1).unwrap() &&
            c[(t.clone().nth(i).unwrap()-97) as usize] == true {
                cnt+=1;
                break;
            } else {
                c[(t.clone().nth(i).unwrap()-97) as usize] = true;
            }
        }
    }
    println!("{}", buf.lines().skip(1).count()-cnt);
}