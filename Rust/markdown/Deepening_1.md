# 💻 Baekjoon Deepening 1 Stage

## Sprout

[Question_Link - 25083](https://www.acmicpc.net/problem/25083)

### Basic Code

```rust
fn main(){
    print!(r#"         ,r'"7
r`-_   ,'  ,/
 \. ". L_r'
   `~\/
      |
      |"#);
}
```

<br>

## King, Queen, Rook, Bishop, Knight, Pawn

[Question_Link - 3003](https://www.acmicpc.net/problem/3003)

### Basic Code

```rust
use std::io;

fn main() {
    const P: [i32; 6] = [1, 1, 2, 2, 2, 8];
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    for (i, v) in
        buf.split_whitespace().map(|x| x.parse::<i32>().unwrap()).enumerate() {
            print!("{} ", P[i]-v);
        }
}
```

<br>

## Draw Stars - 7

[Question_Link - 2444](https://www.acmicpc.net/problem/2444)

### Basic Code

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();
    let mut t = 1;
    for i in 1..=n {
        if i==1 {
            println!("{:>x$}", "*".repeat(t), x=n+(i-1));
        } else {
            println!("{:>x$}", "*".repeat(t+i), x=n+(i-1));
            t = i;
        }
    }
    t = n-1;
    for i in (0..n-1).rev() {
        if i==0 {
            println!("{:>x$}", "*".repeat(t), x=n+i);
        } else {
            println!("{:>x$}", "*".repeat(t+i), x=n+i);
            t = i;
        }
    }
}
```

<br>

### Improvement Code - chain & trim_end

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();
    let w = n*2-1;
    for i in (1..=n).chain((1..n).rev()) {
        let r = format!("{:^w$}", "*".repeat(i*2-1));
        println!("{}", r.trim_end());
    }
}
```

<br>

[trait_Iterator_method_chain in Official Document](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.chain)

[str_method_trim_end in Official Document](https://doc.rust-lang.org/std/primitive.str.html#method.trim_end)

<br>

## Palindrome

[Question_Link - 10988](https://www.acmicpc.net/problem/10988)

### Basic Code

```rust
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    if buf.trim() == buf.trim().chars().rev().collect::<String>() {
        println!("1");
    } else {
        println!("0");
    }
}
```

<br>

## Word Study

[Question_Link - 1157](https://www.acmicpc.net/problem/1157)

### Basic Code

```rust
use std::{io, collections::HashMap};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut h: HashMap<char, u32> = HashMap::new();
    let i = buf.trim().to_ascii_uppercase();
    let mut r = vec![('A', 0)];
    for j in i.chars() {
        let c = h.entry(j).or_insert(0);
        *c+=1;
    }
    for (k, v) in h {
        if v > r[0].1 {
            r.clear();
            r.push((k, v));
        } else if v == r[0].1 {
            r.push((k, v));
        }
    }
    if r.iter().count() > 1 {
        println!("?");
    } else {
        println!("{}", r[0].0);
    }
}
```

<br>

## Croatia Alphabet

[Question_Link - 1157](https://www.acmicpc.net/problem/1157)

### Basic Code

```rust
use std::io;

fn main() {
    const C: [&str; 8] = [
        "c=", "c-", "dz=", "d-",
        "lj", "nj", "s=", "z="
    ];
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut m = String::from(buf.trim());
    for i in C {
        if m.contains(i) {
            m = m.replace(i, "^");
        }
    }
    println!("{}", m.len());
}
```

<br>

## Group Word Checker

[Question_Link - 1316](https://www.acmicpc.net/problem/1316)

### Basic Code

```rust
use std::io::{self, *};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut cnt = 0;
    for w in buf.lines().skip(1) {
        let mut c = [0; 26];
        let t = w.trim().bytes();
        c[(t.clone().nth(0).unwrap()-97) as usize] = 1;
        for i in 1..w.trim().len() {
            if t.clone().nth(i).unwrap() == t.clone().nth(i-1).unwrap() {
                continue;
            } else if t.clone().nth(i).unwrap() != t.clone().nth(i-1).unwrap() &&
            c[(t.clone().nth(i).unwrap()-97) as usize] == 1 {
                cnt+=1;
                break;
            } else {
                c[(t.clone().nth(i).unwrap()-97) as usize] = 1;
            }
        }
    }
    println!("{}", buf.lines().skip(1).count()-cnt);
}
```

<br>

## Your Rating is

[Question_Link - 25206](https://www.acmicpc.net/problem/25206)

### Basic Code

```rust
use std::io::{self, *};

fn make_array(m: &str) -> (Vec<f64>, Vec<f64>) {
    let mut g: Vec<f64> = Vec::new();
    let mut r: Vec<f64> = Vec::new();
    for l in m.lines() {
        let mut t = l.split(' ');
        if t.clone().nth(2).unwrap() == "P" {
            continue;
        } else {
            g.push(t.nth(1).unwrap().parse::<f64>().unwrap());
            r.push(
                match t.nth(0).unwrap() {
                    "A+" => 4.5,
                    "A0" => 4.0,
                    "B+" => 3.5,
                    "B0" => 3.0,
                    "C+" => 2.5,
                    "C0" => 2.0,
                    "D+" => 1.5,
                    "D0" => 1.0,
                    "F" => 0.0,
                    _ => -1.0,
                }
            );
        }
    }
    (g, r)
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut a = 0.0;
    let (g, r) = make_array(buf.trim());
    if g.is_empty() {
        println!("{:.6}", a);
    } else {
        for i in 0..g.len() {
            a+=g[i]*r[i];
        }
        println!("{:.6}", a/g.iter().sum::<f64>());
    }
}
```