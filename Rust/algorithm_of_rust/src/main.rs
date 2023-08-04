use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/data.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let mut v: Vec<Vec<u32>> = Vec::new();
    for l in buf.lines().skip(1) {
        v.push(l.split(" ")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>());
    }
    for i in 0..v.len()/2 {
        for j in 0..v[i].len() {
            print!("{} ", v[i][j]+v[i+v.len()/2][j]);
        }
        println!("");
    }
}