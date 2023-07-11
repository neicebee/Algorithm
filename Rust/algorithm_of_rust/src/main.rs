// use std::io::Read;

// fn main() {
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();

//     let n;
//     let mut b: Vec<_> = Vec::new();

//     {
//         let v: Vec<i32> = buf.lines().next().unwrap().split(' ')
//             .map(|x| x.parse().unwrap()).collect();
//         n = v[0];
//     }

//     (1..=n).for_each(|i| b.push(i));

//     for l in buf.lines().skip(1) {
//         let mut it = l.split(' ').flat_map(str::parse::<i32>);
//         let (mut i, mut j) = (it.next().unwrap()-1, it.next().unwrap()-1);
//         if i!=j {
//             let mut l_p = i;
//             let mut r_p = j;
//             loop {
//                 b.swap(l_p as usize, r_p as usize);
//                 l_p+=1;
//                 r_p-=1;
//                 if r_p==i && l_p==j { break; }
//                 else { i+=1; j-=1; }
//             }
//         }
//     }
//     b.iter().for_each(|x| print!("{x} "));
// }
use std::io::{self, Read};
fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let n;
    let mut b: Vec<_> = Vec::new();
    {
        let v: Vec<i32> = buf.lines().next().unwrap().split(' ')
            .map(|x| x.parse().unwrap()).collect();
        n = v[0];
    }
    (1..=n).for_each(|x| b.push(x));
    
    for l in buf.lines().skip(1) {
        let mut it = l.split(' ').flat_map(str::parse::<i32>);
        let (mut i, mut j) = (it.next().unwrap()-1, it.next().unwrap()-1);
        if i!=j {
            let mut l_p = i;
            let mut r_p = j;
            loop {
                b.swap(l_p as usize, r_p as usize);
                l_p+=1;
                r_p-=1;
                if r_p==i && l_p==j { break; }
                else { i+=1; j-=1; }
            }
        }
    }
    b.iter().for_each(|x| print!("{x} "));
}