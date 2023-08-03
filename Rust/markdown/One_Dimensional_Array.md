# 💻 Baekjoon One Dimensional Array Stage

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
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut n = buf.split_ascii_whitespace().skip(1)
        .flat_map( str::parse::<i32> );
    let x = n.next();

    n.filter(|&i| Some(i) < x).for_each(
        |i| print!("{i} ")
    );
}
```

<br>

## Min, Max

[Question_Link - 10818](https://www.acmicpc.net/problem/10818)

### Basic Code

```rust
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    let mut v = Vec::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    {
        let n = buf.split_ascii_whitespace().skip(1)
            .flat_map( str::parse::<i32> );
        n.for_each(|x| v.push(x));
    }
    let mut max = &v[0];
    let mut min = &v[0];

    v.iter().for_each(
        |x| {
            if max < x { max = x; }
            if min > x { min = x; }
        }
    );

    println!("{min} {max}");
}
```

<br>

### Improvement Code

```rust
use std::io::{self, Read};

fn main() {
    const MAX: i32 = 1_000_000;
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let n = buf.split_ascii_whitespace().skip(1)
        .flat_map( str::parse::<i32> );

    let (min, max) = n.fold(
        (MAX, -MAX), |(min, max), num| {
            (if num < min { num } else { min },
            if num > max { num } else { max })
        });

    // let (min, max) = n.fold(
    //     (MAX, -MAX), |(min, max), num| (num.min(min), num.max(max))
    // );
    // 위와 같은 코드인데 min과 max 메서드를 사용하여 간결하게 짤 수 있음

    println!("{min} {max}");
}
```

<br>

[trait_Iterator_method_fold in Official Document](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)

<br>

## Max Value

[Question_Link - 2562](https://www.acmicpc.net/problem/2562)

### Basic Code

```rust
use std::io::{self, Read};

fn main() {
    const MAX: i32 = 100;
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let n = buf.split('\n').flat_map( str::parse::<i32> );

    let (c, max) = n.enumerate().fold(
        (0, -MAX), |(c, max), (i , v)| {
            if max > v { (c, max) }
            else { (i+1, v.max(max)) }
        });
    
    println!("{max}\n{c}");
}
```

<br>

### Improvement Code - max_by & cmp

```rust
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let n = buf.split_ascii_whitespace().flat_map( str::parse::<i32> );
    let max = n.enumerate().max_by(
        |x, y| (x.1.cmp(&y.1))
    ).unwrap();

    println!("{}\n{}", max.1, max.0+1);
}
```

<br>

[trait_Iterator_method_max_by in Official Document](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.max_by)

[trait_Ord_method_cmp in Official Document](https://doc.rust-lang.org/std/cmp/trait.Ord.html#tymethod.cmp)

<br>

## Put the Ball

[Question_Link - 10810](https://www.acmicpc.net/problem/10810)

### Basic Code

```rust
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let n;

    {
        let v: Vec<i32> = buf.lines().next().unwrap().split(' ').map(
            |x| x.parse().unwrap()
        ).collect();
        n = v[0];
    }

    let mut basket = vec![0; n as usize];

    for l in buf.lines().skip(1) {
        let mut tmp = l.split(' ').flat_map( str::parse::<i32> );
        let (i, j, k) = (
            tmp.next().unwrap()-1,
            tmp.next().unwrap()-1,
            tmp.next().unwrap()
        );
        (i..=j).for_each(|x| {
            basket[x as usize] = k;
        });
    }
    
    basket.iter().for_each(|x| print!("{x} "));
}
```

<br>

## Change the Ball

[Question_Link - 10810](https://www.acmicpc.net/problem/10810)

### Basic Code

```rust
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let n;
    let mut b: Vec<i32> = Vec::new();

    {
        let v: Vec<i32> = buf.lines().next().unwrap().split(' ')
            .map(|x| x.parse().unwrap()).collect();
        n = v[0];
    }

    (0..n).for_each(|x| b.push(x+1));

    for l in buf.lines().skip(1) {
        let mut t = l.split(' ').flat_map(str::parse::<i32>);
        let (i, j) = (t.next().unwrap()-1, t.next().unwrap()-1);
        b.swap(i as usize, j as usize);
        // let tmp = b[i as usize];
        // b[i as usize] = b[j as usize];
        // b[j as usize] = tmp;
        // 해당 코드와 같은 동작을 하는 벡터의 swap 메서드
    }
    b.iter().for_each(|x| print!("{x} "));
}
```

<br>

## Who hasn't submitted an assignment?

[Question_Link - 5597](https://www.acmicpc.net/problem/5597)

### Basic Code

```rust
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split('\n').flat_map(str::parse::<u8>);
    let sub = [0; 28].map(|_| iter.next().unwrap());
    (1..=30).for_each(|x| if !sub.contains(&x) { println!("{x}") });
}
```

<br>

### Improvement Code

```rust
use std::io::{stdin, BufReader, BufRead};

fn main() {
    let s: Vec<u8> = BufReader::new(stdin()).lines()
        .map(|x| x.unwrap().parse::<u8>().unwrap()).collect();
    (1..=30).for_each(|i| if !s.contains(&i) { println!("{i}") });
}
```

<br>

[struct_BufReader in Official Document](https://doc.rust-lang.org/std/io/struct.BufReader.html)

[trait_BufRead in Official Document](https://doc.rust-lang.org/std/io/trait.BufRead.html)

<br>

## Modulo

[Question_Link - 3052](https://www.acmicpc.net/problem/3052)

### Basic Code

```rust
use std::io::{stdin, BufReader, BufRead};

fn main() {
    let mut buf: Vec<u32> = BufReader::new(stdin()).lines()
        .map(|x| x.unwrap().parse::<u32>().unwrap()%42).collect();
    buf.sort();
    buf.dedup();
    println!("{}", buf.len());
}
```

<br>

## Reverse the Basket

[Question_Link - 10811](https://www.acmicpc.net/problem/10811)

### Basic Code - swap

```rust
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let n: u32;
    {
        let mut it = buf.lines().next().unwrap().split(' ')
            .map(|x| x.parse().unwrap());
        n = it.next().unwrap();
    }
    let mut b: Vec<_> = Vec::new();
    (1..=n).for_each(|x| b.push(x));
    
    for l in buf.lines().skip(1) {
        let mut it = l.split(' ').flat_map(str::parse::<u32>);
        let (i, j) = (it.next().unwrap()-1, it.next().unwrap()-1);
        (0..=(j-i)/2).for_each(
            |t| b.swap((t+i) as usize, (j-t) as usize)
        );
    }
    b.iter().for_each(|i| print!("{i} "));
}
```

<br>

### Basic Code - reverse

```rust
use std::io::{self, Read};

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let n: usize;
    {
        let mut it = buf.lines().next().unwrap().split(' ')
            .map(|x| x.parse().unwrap());
        n = it.next().unwrap();
    }
    let mut b: Vec<_> = Vec::new();
    (1..=n).for_each(|x| b.push(x));
    
    for l in buf.lines().skip(1) {
        let mut it = l.split(' ').flat_map(str::parse::<usize>);
        let (i, j) = (it.next().unwrap()-1, it.next().unwrap()-1);
        b[i..=j].reverse();
    }
    b.iter().for_each(|i| print!("{i} "));
}
```

<br>

## Average

[Question_Link - 1546](https://www.acmicpc.net/problem/1546)

### Basic Code

```rust
use std::io::{self, Read};

fn main() {
    const MAX: f64 = 100.0;
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let s: Vec<f64> = buf.lines().skip(1).next().unwrap()
        .split(' ').map(|x| x.parse().unwrap()).collect();
    let max = s.iter().fold(
        -MAX, |max, n| n.max(max));
    let mut r: Vec<f64> = Vec::new();
    s.iter().for_each(|x| r.push(x/max*100.0));
    println!("{}", r.iter().sum::<f64>()/r.len() as f64);
}
```