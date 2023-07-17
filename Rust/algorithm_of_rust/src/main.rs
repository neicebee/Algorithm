use std::io::{self, Read};

fn main() {
    let mut f = std::fs::File::open("src/data.txt").unwrap();
    let mut buf = String::new();
    let mut v: Vec<i32> = Vec::new();
    f.read_to_string(&mut buf).unwrap();
    let mut iter = buf.lines();
    
    let (n, m) = (iter.next().unwrap().parse::<usize>().unwrap()
        , iter.next().unwrap());
    (1..=n).for_each(|x| {
        let tmp = &m[x-1..x];
        println!("{}", tmp);
        v.push(tmp.parse::<i32>().clone().unwrap());
    });

    println!("{}", v.iter().sum::<i32>());
}