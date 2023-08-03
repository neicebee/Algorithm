// use std::io::Read;

// fn make_array(m: &str) -> (Vec<f64>, Vec<f64>) {
//     let mut g: Vec<f64> = Vec::new();
//     let mut r: Vec<f64> = Vec::new();
//     for l in m.lines() {
//         let mut t = l.split(' ');
//         if t.clone().nth(2).unwrap() == "P" {
//             continue;
//         } else {
//             g.push(t.nth(1).unwrap().parse::<f64>().unwrap());
//             r.push(
//                 match t.nth(0).unwrap() {
//                     "A+" => 4.5,
//                     "A0" => 4.0,
//                     "B+" => 3.5,
//                     "B0" => 3.0,
//                     "C+" => 2.5,
//                     "C0" => 2.0,
//                     "D+" => 1.5,
//                     "D0" => 1.0,
//                     "F" => 0.0,
//                     _ => -1.0,
//                 }
//             );
//         }
//     }
//     (g, r)
// }

// fn main() {
//     let mut f = std::fs::File::open("src/data.txt").unwrap();
//     let mut buf = String::new();
//     f.read_to_string(&mut buf).unwrap();
//     let mut a = 0.0;
//     let (g, r) = make_array(buf.trim()); 
//     if g.is_empty() && r.is_empty() {
//         println!("{:.6}", a);
//     } else {
//         for i in 0..g.len() {
//             a+=g[i]*r[i];
//         }
//         println!("{:.6}", a/g.iter().sum::<f64>());
//     }
// }

use std::io::{self, *};

fn make_array(m: &str) -> (Vec<f64>, Vec<f64>) {
    let mut g: Vec<f64> = Vec::new();
    let mut r: Vec<f64> = Vec::new();
    for l in m.lines() {
        let mut t = l.split(' ');
        if t.clone().nth(2).unwrap() == "P" {
            continue;
        } else {
            g.push(t.nth(1).unwrap().parse::<f64>().unwrap());
            r.push(
                match t.nth(0).unwrap() {
                    "A+" => 4.5,
                    "A0" => 4.0,
                    "B+" => 3.5,
                    "B0" => 3.0,
                    "C+" => 2.5,
                    "C0" => 2.0,
                    "D+" => 1.5,
                    "D0" => 1.0,
                    "F" => 0.0,
                    _ => -1.0,
                }
            );
        }
    }
    (g, r)
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut a = 0.0;
    let (g, r) = make_array(buf.trim());
    if g.is_empty() {
        println!("{:.6}", a);
    } else {
        for i in 0..g.len() {
            a+=g[i]*r[i];
        }
        println!("{:.6}", a/g.iter().sum::<f64>());
    }
}