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
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    for l in buf.lines().skip(1) {
        println!("{}{}", &l[..1], &l[l.len()-1..]);
    }
}
```

<br>

## ASCII Code

[Question_Link - 11654](https://www.acmicpc.net/problem/11654)

### Basic Code

```rust
use std::io::{self, Read};

fn main() {
    let mut b = [0; 1];
    io::stdin().read(&mut b).unwrap();
    println!("{}", &b[0]);
}
```

<br>

## Sum of Numbers

[Question_Link - 11720](https://www.acmicpc.net/problem/11720)

### Basic Code

```rust
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut v: Vec<i32> = Vec::new();
    let mut iter = buf.lines();
    let (n, m) = (iter.next().unwrap().parse::<usize>().unwrap(),
        iter.next().unwrap());
    (1..=n).for_each(|x| {
        let tmp = &m[x-1..x];
        v.push(tmp.parse::<i32>().clone().unwrap());
    });
    println!("{}", v.iter().sum::<i32>());
}
```

<br>

### Improvement Code - chars & to_digit

```rust
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    for l in buf.lines().skip(1) {
        let s = l.chars().map(|x| x.to_digit(10).unwrap())
            .sum::<u32>();
        println!("{s}");
    }
}
```

<br>

## Find the Alphabet

[Question_Link - 10809](https://www.acmicpc.net/problem/10809)

### Basic Code

```rust

```