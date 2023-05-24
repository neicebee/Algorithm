use std::io::{self, Write};

fn main() {
    let mut out = io::BufWriter::new(io::stdout());
    for l in io::stdin().lines() {
        let r = l.unwrap().trim().split(' ').map(
            |x| x.parse::<i32>().unwrap()
        ).sum::<i32>();
        writeln!(out, "{r}").unwrap();
    }
}