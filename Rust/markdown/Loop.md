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
    (1..=n).for_each(
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
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)
        .unwrap();
    buf.pop();
    let n = buf.parse::<i32>().unwrap();
    println!("{}", n*(n+1)/2)
}
```

<br>

## Receipt

[Question_Link - 25304](https://www.acmicpc.net/problem/25304)

### Basic Code

```rust
use std::io;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)
        .unwrap();
    buf.pop();
    buf
}

fn main() {
    let x = input().parse::<i32>().unwrap();
    let n = input().parse::<i32>().unwrap();
    let mut s = 0;
    (0..n).for_each(
        |_| {
            let mut tmp = 1;
            input().split(' ').for_each(
                |y| {
                    tmp*=y.parse::<i32>().unwrap()
                }
            );
            s+=tmp;
        }
    );
    println!("{}", if x==s { "Yes" } else { "No" });
}
```

<br>

## Coding is Physical Education

[Question_Link - 25314](https://www.acmicpc.net/problem/25314)

### Basic Code - 1

```rust
use std::io;

fn main() {
    let mut s = String::new();
    let l = "long ";
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let n = {
        (buf.parse::<i32>().unwrap())/4
    };
    (0..n).for_each(
        |_| s.push_str(l)
    );
    println!("{}int", s);
}
```

<br>

### Basic Code - 2

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let n = buf.parse::<i32>().unwrap();
    for _ in 0..n>>2 {
        print!("long ");
    }
    print!("int");
}
```

<br>

## Quick A+B

[Question_Link - 15552](https://www.acmicpc.net/problem/15552)

### Basic Code

```rust
use std::io::{self, Write};

fn main() {
    let mut out = io::BufWriter::new(io::stdout());
    for l in io::stdin().lines().skip(1) {
        let r = l.unwrap().split(' ').map(
            |x| x.parse::<i32>().unwrap()
        ).sum::<i32>();
        writeln!(out, "{}", r).unwrap();
    }
}
```

<br>

### `lines()` & `skip()`

```
// data.txt
5
1 1
12 34
5 500
40 60
1000 1000
```

```rust
use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("src/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for l in contents.lines().skip(1) {
        print!("{l}: ");
        let r = l.split(' ').map(
            |x| x.parse::<i32>().unwrap()
        ).sum::<i32>();
        println!("{r}");
    }
}
// Result
// 1 1: 2
// 12 34: 46
// 5 500: 505
// 40 60: 100
// 1000 1000: 2000
```

<br>

## A+B - 7

[Question_Link - 11021](https://www.acmicpc.net/problem/11021)

### Basic Code

```rust
use std::io::{self, Write};

fn main() {
    let mut cnt: i32 = 1;
    let mut out = io::BufWriter::new(io::stdout());
    for l in io::stdin().lines().skip(1) {
        let r = l.unwrap().split(' ').map(
            |x| x.parse::<i32>().unwrap()
        ).sum::<i32>();
        writeln!(out, "Case #{cnt}: {r}").unwrap();
        cnt+=1;
    }
}
```

<br>

## A+B - 8

[Question_Link - 11022](https://www.acmicpc.net/problem/11022)

### Basic Code

```rust
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
```

<br>

## Draw Stars - 1

[Question_Link - 2438](https://www.acmicpc.net/problem/2438)

### Basic Code - for & for_each

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    for i in 1..=buf.parse::<i32>().unwrap() {
        (0..i).for_each(|_| print!("*"));
        print!("\n");
    }
}
```

<br>

### Basic Code - only for_each

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    (1..=buf.parse::<usize>().unwrap()).for_each(
        |x| println!("{}", "*".repeat(x))
    );
}
```

<br>

## Draw Stars - 2

[Question_Link - 2439](https://www.acmicpc.net/problem/2439)

### Basic Code

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let n = buf.parse::<usize>().unwrap();
    (1..=n).for_each(
        |x| println!(
            "{}{}", " ".repeat(n-x), "*".repeat(x)
        )
    );
}
```

<br>

## A+B - 5

[Question_Link - 10952](https://www.acmicpc.net/problem/10952)

### Basic Code

```rust
use std::io::{self, Write};

fn main() {
    let mut out = io::BufWriter::new(io::stdout());
    for l in io::stdin().lines() {
        let r = l.unwrap().trim().split(' ').map(
            |x| x.parse::<i32>().unwrap()
        ).sum::<i32>();
        if r != 0 {
            writeln!(out, "{r}").unwrap();
        }
    }
}
```

<br>

## A+B - 4

[Question_Link - 10951](https://www.acmicpc.net/problem/10951)

### Basic Code

```rust
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
```