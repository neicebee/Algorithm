use std::io::Read;

fn main() {
    let mut f = std::fs::File::open("src/data.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();

    let mut content = buf.lines().next();

    println!("{:?}", content);
}