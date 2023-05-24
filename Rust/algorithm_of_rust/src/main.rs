use std::io::{self, Write};

fn main() {
    let mut cnt = 1;
    let mut out = io::BufWriter::new(io::stdout());
    for l in io::stdin().lines().skip(1) {
        let r: Vec<i32> = l.unwrap().split(' ').map(
            |x| x.parse::<i32>().unwrap()
        ).collect();
        writeln!(
            out, "Case #{cnt}: {} + {} = {}",
            r[0], r[1], r.iter().sum::<i32>()
        ).unwrap();
        cnt+=1;
    }
}