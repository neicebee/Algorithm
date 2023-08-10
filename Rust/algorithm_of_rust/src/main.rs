// use std::io::Read;

// fn main() {
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();
//     let mut p = [[0; 100]; 100];
//     let mut r = 0;
//     for l in buf.lines().skip(1) {
//         let (x, y) = {
//             let mut t = l.split(' ');
//             (t.next().unwrap().parse::<usize>().unwrap(), 
//             t.next().unwrap().parse::<usize>().unwrap())
//         };
//         for i in x..x+10 {
//             for j in y..y+10 {
//                 if p[i][j] == 0 {
//                     p[i][j] = 1;
//                     r+=1;
//                 }
//             }
//         }
//     }
//     println!("{r}");
// }

use std::io::{self, *};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut p = [[0; 100]; 100];
    let mut r = 0;
    for l in buf.lines().skip(1) {
        let (x, y) = {
            let mut t = l.split(' ');
            (t.next().unwrap().parse::<usize>().unwrap(),
            t.next().unwrap().parse::<usize>().unwrap())
        };
        for i in x..x+10 {
            for j in y..y+10 {
                if p[i][j]==0 {
                    p[i][j]=1;
                    r+=1;
                }
            }
        }
    }
    println!("{r}");
}