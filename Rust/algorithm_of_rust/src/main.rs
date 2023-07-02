// use std::io::{self, Read};

// fn main() {
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut c = String::new();
//     f.read_to_string(&mut c).unwrap();

//     let n = c.split_ascii_whitespace().skip(1)
//         .flat_map( str::parse::<i32> );

//     const MAX: i32 = 1_000_000;

//     let (min, max) = n.fold(
//         (MAX, -MAX), |(min, max), num| {
//             (if num < min { num } else { min }, 
//             if num > max { num } else { max })
//         });
    
//     println!("{min} {max}");
// }

use std::io::{self, Read};

fn main() {
    const MAX: i32 = 1_000_000;
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let n = buf.split_ascii_whitespace().skip(1)
        .flat_map( str::parse::<i32> );

    let (min, max) = n.fold(
        (MAX, -MAX), |(min, max), num| {
            (if num < min { num } else { min },
            if num > max { num } else { max })
        });

    // let (min, max) = n.fold(
    //     (MAX, -MAX), |(min, max), num| (num.min(num), num.max(num))
    // );

    println!("{min} {max}");
}