// use std::io::Read;

// fn main() {
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();

//     let n;
//     let mut v: Vec<i32> = Vec::new();

//     {
//         let v: Vec<i32> = buf.lines().next().unwrap().split(' ')
//             .map(|x| x.parse().unwrap() ).collect();
//         n = v[0];
//     }

//     (0..n).for_each(|x| v.push(x+1));

//     for l in buf.lines().skip(1) {
//         let mut t = l.split(' ').flat_map( str::parse::<i32> );
//         let (i, j) = (t.next().unwrap()-1, t.next().unwrap()-1);
        // let tmp = b[i as usize];
        // b[i as usize] = b[j as usize];
        // b[j as usize] = tmp;
//         v.swap(i as usize, j as usize);
//     }

//     v.iter().for_each(|x| print!("{x} "));
// }

use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let n;
    let mut b: Vec<i32> = Vec::new();

    {
        let v: Vec<i32> = buf.lines().next().unwrap().split(' ')
            .map(|x| x.parse().unwrap()).collect();
        n = v[0];
    }

    (0..n).for_each(|x| b.push(x+1));

    for l in buf.lines().skip(1) {
        let mut t = l.split(' ').flat_map(str::parse::<i32>);
        let (i, j) = (t.next().unwrap()-1, t.next().unwrap()-1);
        b.swap(i as usize, j as usize);
    }
    b.iter().for_each(|x| print!("{x} "));
}