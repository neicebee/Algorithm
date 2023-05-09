# 💻 Baekjoon Loop Stage

## Multiplication Table

[Question_Link - 2739](https://www.acmicpc.net/problem/2739)

### Basic Code

```rust
use std::io;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    buf
}

fn main() {
    let n: i32 = input().parse().unwrap();
    for i in 1..10 {
        println!("{n} * {i} = {}", n*i);
    }
}
```

<br>

## A+B - 3

[Question_Link - 10950](https://www.acmicpc.net/problem/10950)

### Basic Code

```rust
use std::io;

fn result() -> i32 {
    let mut buf = String::new();
    let mut r = 0;
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    buf.split(' ')
        .for_each(|x| r+=x.parse::<i32>().unwrap());
    r
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let t = buf.parse::<i32>().unwrap();
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..t {
        v.push(result());
    }
    v.iter()
        .for_each(|x| println!("{x}"));
}
```

<br>

## Sum

[Question_Link - 8393](https://www.acmicpc.net/problem/8393)

### Basic Code - for_each

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)
        .unwrap();
    buf.pop();
    let n = buf.parse::<i32>().unwrap();
    let mut s = 0;
    (1..n+1).for_each(
        |x| {
            s+=x;
        }
    );
    println!("{s}");
}
```

<br>

### Basic Code - formula

```rust

```