# 💻 Baekjoon String Stage

## Character & String

[Question_Link - 27866](https://www.acmicpc.net/problem/27866)

### Basic Code

```rust
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.lines();
    let s = String::from(iter.next().unwrap());
    let i = iter.next().unwrap().parse::<usize>().unwrap();
    println!("{}", &s[i-1..i]);
}
```

<br>

## Measure Word Length

[Question_Link - 2743](https://www.acmicpc.net/problem/2743)

### Basic Code

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    println!("{}", buf.trim().len());
}
```

<br>

## String

[Question_Link - 9086](https://www.acmicpc.net/problem/9086)

### Basic Code

```rust

```