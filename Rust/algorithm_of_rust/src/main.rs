// use std::io::Read;

// fn main() {
//     const C: [u32; 4] = [25, 10, 5, 1];
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();
//     for l in buf.lines().skip(1) {
//         let mut c = l.trim().parse::<u32>().unwrap();
//         for i in C {
//             print!("{} ", c/i);
//             c%=i;
//         }
//         println!("");
//     }
// }

use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<u32>().unwrap();
    println!("{}", (1+2_u32.pow(n)).pow(2));
}