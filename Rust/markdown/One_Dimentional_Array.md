# 💻 Baekjoon One Dimentional Array Stage

## Count Number

[Question_Link - 10807](https://www.acmicpc.net/problem/10807)

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
    let mut n: Vec<i32> = Vec::new();
    let mut v = 0;
    (0..3).for_each(
        |x| {
            let s = input();
            if x==1 {
                n = s.split(" ").map(
                    |y| y.parse().unwrap()
                ).collect();
            } else if x==2 {
                v = s.parse().unwrap();
            }
        }
    );
    println!("{}", count_number(n, v));
}

fn count_number(l: Vec<i32>, v: i32) -> i32 {
    let mut cnt = 0;
    for i in l {
        if i==v {
            cnt+=1;
        }
    }
    cnt
}
```

<br>

### Improvement Code - flat_map & filter

```rust
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut n = buf.split_ascii_whitespace().flat_map( str::parse::<i32> );
    let (_, v) = (n.next(), n.next_back().unwrap());
    let cnt = n.filter(|&x| x==v).count();
    println!("{cnt}");
}
```

<br>

[trait_Iterator_method_flat_map in Official Document](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map)

[trait_Iterator_method_filter in Official Document](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)

<br>

## Number less than X

[Question_Link - 10871](https://www.acmicpc.net/problem/10871)

### Basic Code

```rust

```