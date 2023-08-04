# 💻 Baekjoon Two Dimensional Array Stage

## Matrix Sum

[Question_Link - 2738](https://www.acmicpc.net/problem/2738)

### Basic Code

```rust
use std::io::{self, *};

fn make_matrix(n: u32, m: &str) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut v: Vec<Vec<Vec<u32>>> = Vec::new();
    let mut iter = m.lines().skip(1);
    for _ in 0..iter.clone().count() as u32/n {
        let mut t: Vec<Vec<u32>> = Vec::new();
        (0..n).for_each(|_| {
            t.push(iter.next().unwrap().split_whitespace()
                .map(|y| y.parse::<u32>().unwrap()).collect::<Vec<u32>>());
        });
        v.push(t);
    }
    (v[0].to_owned(), v[1].to_owned())
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let n = buf.lines().next().unwrap()
        .split_whitespace().next().unwrap().parse::<u32>().unwrap();
    if n == 0 {
        println!("0");
        return;
    }
    let (a, b) = make_matrix(n, buf.trim());
    let mut r: Vec<Vec<u32>> = Vec::new();
    for i in 0..n {
        let mut t: Vec<u32> = Vec::new();
        for j in 0..a[i as usize].clone().len() {
            t.push(a[i as usize][j]+b[i as usize][j]);
        }
        r.push(t);
    }
    for i in r.iter() {
        i.iter().for_each(|x| print!("{} ", x));
        println!("");
    }
}
```

<br>

### Improvement Code

```rust
use std::io::{self, *};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
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
```

<br>

## Matrix Sum

[Question_Link - 2738](https://www.acmicpc.net/problem/2738)

### Basic Code