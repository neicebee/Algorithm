// use std::io::Read;

// fn main() {
// 	let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();
    
// }

use std::{io, collections::HashMap};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut h: HashMap<char, u32> = HashMap::new();
    let i = buf.trim().to_ascii_uppercase();
    let mut r = vec![('A', 0)];
    for j in i.chars() {
        let c = h.entry(j).or_insert(0);
        *c+=1;
    }
    for (k, v) in h {
        if v > r[0].1 {
            r.clear();
            r.push((k, v));
        } else if v == r[0].1 {
            r.push((k, v));
        }
    }
    if r.iter().count() > 1 {
        println!("?");
    } else {
        println!("{}", r[0].0);
    }
}