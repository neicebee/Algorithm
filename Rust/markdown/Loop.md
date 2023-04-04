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

```