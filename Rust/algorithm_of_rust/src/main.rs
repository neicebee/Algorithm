// use std::io::Read;

// fn main() {
//     const PIECE: [i32; 6] = [1, 1, 2, 2, 2, 8];
// 	let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();
//     for (i, v) in 
//         buf.split_whitespace().map(|x| x.parse::<i32>().unwrap()).enumerate() {
//             print!("{} ", PIECE[i]-v);
//     }
// }

use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();
    let mut t = 1;
    for i in 1..=n {
        if i==1 {
            println!("{:>x$}", "*".repeat(t), x=n+(i-1));
        } else {
            println!("{:>x$}", "*".repeat(t+i), x=n+(i-1));
            t = i;
        }
    }
    t = n-1;
    for i in (0..n-1).rev() {
        if i==0 {
            println!("{:>x$}", "*".repeat(t), x=n+i);
        } else {
            println!("{:>x$}", "*".repeat(t+i), x=n+i);
            t = i;
        }
    }
}