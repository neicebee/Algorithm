# 💻 Baekjoon Condition Stage

## Two Number Compare

[Question_Link - 1330](https://www.acmicpc.net/problem/1330)

### Basic Code

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let nums: Vec<i64> = buf.trim().split(' ')
        .map(
            |x| x.parse().unwrap()
        ).collect();
    if nums[0]>nums[1] {
        println!(">");
    } else if nums[0]<nums[1] {
        println!("<");
    } else {
        println!("==");
    }
}
```

<br>

## Test Score

[Question_Link - 9498](https://www.acmicpc.net/problem/9498)

### Basic Code - if

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let tmp: u16 = buf.trim().parse().unwrap();
    let score = if 90<=tmp && tmp<=100 {
        'A'
    } else if 80<=tmp && tmp<=89 {
        'B'
    } else if 70<=tmp && tmp<=79 {
        'C'
    } else if 60<=tmp && tmp<=69 {
        'D'
    } else {
        'F'
    };
    println!("{score}");
}
```

### Basic Code - match

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let tmp: u16 = buf.trim().parse().unwrap();
    let score = match tmp {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };
    println!("{score}");
}
```

<br>

## Leap Year

[Question_Link - 2753](https://www.acmicpc.net/problem/2753)

### Basic Code

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let y: u32 = buf.trim().parse().unwrap();
    let l = if y%4==0 && y%100!=0 {
        1
    } else if y%400==0 {
        1
    } else {
        0
    };
    println!("{l}");
}
```

### Improvement Code

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let y: u32 = buf.trim().parse().unwrap();
    let l = ((y%4==0 && y%100!=0) || y%400==0) as u32;
    println!("{l}");
}
```

<br>

## Choose Quadrant

[Question_Link - 14681](https://www.acmicpc.net/problem/14681)

### Basic Code

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut buf2 = String::new();
    io::stdin().read_line(&mut buf2).unwrap();
    let x: i32 = buf.trim().parse().unwrap();
    let y: i32 = buf2.trim().parse().unwrap();
    let q = if x>0 && y>0 {
        1
    } else if x<0 && y>0 {
        2
    } else if x<0 && y<0 {
        3
    } else {
        4
    };
    println!("{q}");
}
```

### Improvement Code

```rust
use std::io;

fn main() {
    let t: Vec<_> = (0..2)
        .map(
            |_| {
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                buf.trim().parse::<i32>().unwrap() > 0
            }
        ).collect();
    let q = match t[..] {
        [true,true] => 1,
        [false,true] => 2,
        [false,false] => 3,
        _ => 4,
    };
    println!("{q}");
}
```

<br>

## Alarm

[Question_Link - 2884](https://www.acmicpc.net/problem/2884)

### Basic Code

```rust

```