# 💻 Baekjoon Factors, Multiples and Decimal Stage

## Multiples and Factors

[Question_Link - 5086](https://www.acmicpc.net/problem/5086)

### Basic Code

```rust
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    for l in buf.lines() {
        if l.contains("0 0") { break; }
        let (a, b) = {
            let mut tmp = l.trim().split_whitespace().map(
                |x| x.parse::<u32>().unwrap()
            );
            (tmp.next().unwrap(), tmp.next().unwrap())
        };
        if b%a==0 {
            println!("factor");
        } else if a%b==0 {
            println!("multiple");
        } else {
            println!("neither");
        }
    }
}
```

<br>

## Get Factors

[Question_Link - 2501](https://www.acmicpc.net/problem/2501)

### Basic Code

```rust
use std::{io, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let (n, k) = {
        let mut t = buf.trim().split_whitespace()
            .map(|x| x.parse::<u32>().unwrap());
        (t.next().unwrap(), t.next().unwrap())
    };
    let factors: Vec<u32> = (1..=n).filter(|x| n%x==0).collect();
    if factors.len()<k as usize { println!("0"); }
    else { 
        println!("{}",
            factors[k as usize - 1]
        );
    }
    Ok(())
}
```

<br>

## Sum of Factors

[Question_Link - 9506](https://www.acmicpc.net/problem/9506)

### Basic Code

```rust
use std::{io, io::Read, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    for l in buf.lines() {
        if l=="-1" { break; }
        let a = l.trim().parse::<u32>().unwrap();
        let mut f: Vec<u32> = (1..=a).filter(|x| a%x==0).collect();
        f.pop();
        if a==f.iter().sum() {
            println!("{a} = {}",
                f.iter().map(
                    |x| x.to_string()
                ).collect::<Vec<String>>().join(" + ")
            );
        } else {
            println!("{a} is NOT perfect.");
        }
    }
    Ok(())
}
```