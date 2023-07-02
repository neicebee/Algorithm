// use std::io::Read;

// fn main() {
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut c = String::new();
//     const MAX: i32 = 100;
//     f.read_to_string(&mut c).unwrap();
    
//     let n = c.split('\n').flat_map( str::parse::<i32> );

//     let (i, max) = n.enumerate().fold(
//         (0, -MAX), |(i, max), (j, v)| {
//             if max > v { (i, max) }
//             else { (j+1, v.max(max)) }
//         });
    
//     println!("{max}\n{i}");
// }

use std::io::{self, Read};

fn main() {
    const MAX: i32 = 100;
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let n = buf.split('\n').flat_map( str::parse::<i32> );

    let (c, max) = n.enumerate().fold(
        (0, -MAX), |(c, max), (i , v)| {
            if max > v { (c, max) }
            else { (i+1, v.max(max)) }
        });
    
    println!("{max}\n{c}");
}