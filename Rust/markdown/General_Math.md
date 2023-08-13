# 💻 Baekjoon General Math Stage

## Arithmetic Conversion

[Question_Link - 2745](https://www.acmicpc.net/problem/2745)

### Basic Code

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let (b, n) = {
        let mut t = buf.trim().split(' ');
        (t.next().unwrap(), t.next().unwrap().parse::<u32>().unwrap())
    };
    println!("{}", u64::from_str_radix(b, n).unwrap());
}
```

### Additional Code

```rust
use std::io;

// 진수와 곱하고 각 자리 수 숫자와 더해감
fn convert(n: &str, b: u32) -> u32 {
    let nums = n.chars().collect::<Vec<char>>();
    let mut r = 0;
    for i in 0..nums.len() {
        if nums[i] >= 'A' {
            r = r * b + (nums[i] as u8 - 'A' as u8 + 10) as u32;
        } else {
            r = r * b + (nums[i] as u8 - '0' as u8) as u32;
        }
    }
    r
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let (n, b) = {
        let mut t = buf.trim().split(' ');
        (t.next().unwrap(), t.next().unwrap().parse::<u32>().unwrap())
    };
    println!("{}", convert(n, b));
}
```

<br>

## Arithmetic Conversion 2

[Question_Link - 11005](https://www.acmicpc.net/problem/11005)

### Basic Code

```rust
use std::io;

// 10진법으로 표기된 숫자 n을 나누어 그 나머지를 표시하고 더 이상 나눌 수 없을 때까지 반복하여 표기함
fn convert(n: u32, b: u32) -> String {
    let mut r = String::new();
    let mut c = n;
    loop {
        if c <= 0 { break; }
        if c%b < 10 {
            r.push_str(&(c%b).to_string());
        } else {
            r.push(((c%b - 10 + 65) as u8) as char);
        }
        c /= b;
    }
    r.chars().rev().collect::<String>()
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let (n, b) = {
        let mut t = buf.trim().split(' ');
        (t.next().unwrap().parse::<u32>().unwrap(), 
        t.next().unwrap().parse::<u32>().unwrap())
    };
    println!("{}", convert(n, b));
}
```

<br>

## 세탁소 사장 동혁

[Question_Link - 2720](https://www.acmicpc.net/problem/2720)

### Basic Code

```rust
use std::io::{self, *};

fn main() {
    const C: [u32; 4] = [25, 10, 5, 1];
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    for l in buf.lines().skip(1) {
        let mut c = l.trim().parse::<u32>().unwrap();
        for i in C {
            print!("{} ", c/i);
            c%=i;
        }
        println!("");
    }
}
```

<br>

## 중앙 이동 알고리즘

[Question_Link - 2903](https://www.acmicpc.net/problem/2903)

### Basic Code

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<u32>().unwrap();
    println!("{}", (1+2_u32.pow(n)).pow(2));
}
```
- 공식 : $(1+2^n)^2$

<br>

## 벌집

[Question_Link - 2292](https://www.acmicpc.net/problem/2292)

### Basic Code

```rust

```

-https://blog.naver.com/kwonhj214/223072847019