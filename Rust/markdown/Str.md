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
use std::io::{self, *};

fn main() {
    let mut l = [-1_i32; 26];
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    for (i, v) in buf.trim().bytes().enumerate() {
        if l[(v-97) as usize] == -1 {
            l[(v-97) as usize] = i as i32;
        }
    }
    l.iter().for_each(|x| print!("{x} "));
}
```

<br>

## Repeat String

[Question_Link - 2675](https://www.acmicpc.net/problem/2675)

### Basic Code

```rust
use std::io::{self, *};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    for l in buf.lines().skip(1) {
        let mut sp = l.split(' ');
        let (r, s) = (
            sp.next().unwrap().parse::<u8>().unwrap(),
            sp.next().unwrap().chars());
        s.for_each(|x| (0..r).for_each(|_| print!("{x}")));
        println!("");
    }
}
```

<br>

## Number of words

[Question_Link - 1152](https://www.acmicpc.net/problem/1152)

### Basic Code - split()

```rust
use std::io::{self, *};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    match buf.trim().len() {
        0 => println!("0"),
        _ => println!("{}", buf.trim().split(' ').count()),
    }
}
```

### Basic Code - split_whitespace()

```rust
use std::io::{self, *};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    println!("{}", buf.split_whitespace().count());
}
```

<br>

## 상수

[Question_Link - 2908](https://www.acmicpc.net/problem/2908)

### Basic Code

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let m: i32 = buf.split_whitespace()
        .map(|x| x.chars().rev().collect::<String>())
        .map(|x| x.parse().unwrap()).max().unwrap();
    println!("{m}");
}
```

<br>

## Dial

[Question_Link - 5622](https://www.acmicpc.net/problem/5622)

### Basic Code

```rust
use std::{io, collections::HashMap};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut h: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut t = 65;
    let mut r = 0;
    (3..=10).for_each(|x| {
        let mut t2: Vec<u8> = Vec::new();
        if x!=8 && x!=10 {
            (0..3).for_each(|_| {t2.push(t); t+=1;});
            h.insert(x, t2);
        } else {
            (0..4).for_each(|_| {t2.push(t); t+=1;});
            h.insert(x, t2);
        }
    });
    for i in buf.trim().bytes() {
        for (k, v) in h.iter() {
            if v.contains(&i) {
                r+=k;
            }
        }
    }
    println!("{r}");
}
```

<br>

### Improvement Code

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let r = buf.trim().bytes().map(
        |x| {
            match x-65 {
                0..=17 => (x-65)/3+3,
                18 => 8,
                19..=21 => 9,
                22..=25 => 10,
                _ => 0,
            }
        }
    ).sum::<u8>();
    println!("{r}");
}
```

<br>

## Output as is

[Question_Link - 11718](https://www.acmicpc.net/problem/11718)

### Basic Code

```rust
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    println!("{buf}");
}
```