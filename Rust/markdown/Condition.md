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
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut time: Vec<i32> = buf.trim().split(' ')
        .map(
            |x| x.parse().unwrap()
        ).collect();
    if time[1]-45 < 0 { 
        if time[0]-1 < 0 {
            time[0] = 23;
        } else {
            time[0] -= 1; 
        }
        time[1] = (time[1]+60)-45;
    } else {
        time[1] = time[1]-45;
    }
    println!("{} {}", time[0], time[1]);
}
```

<br>

## Oven Clock

[Question_Link - 2525](https://www.acmicpc.net/problem/2525)

### Basic Code

```rust
use std::io;

fn main() {
    let mut buf1 = String::new();
    io::stdin().read_line(&mut buf1).unwrap();
    let mut t: Vec<i32> = buf1.trim().split(' ')
        .map(
            |x| x.parse().unwrap()
        ).collect();
    let mut buf2 = String::new();
    io::stdin().read_line(&mut buf2).unwrap();
    let c: i32 = buf2.trim().parse().unwrap();
    
    if t[1]+c >= 60 {
        if t[0]+((t[1]+c)/60) > 23 {
            t[0]+=((t[1]+c)/60)-24;
        } else {
            t[0]+=(t[1]+c)/60;
        }
        t[1]=(t[1]+c)%60
    } else {
        t[1]+=c;
    }
    println!("{} {}", t[0], t[1]);
}
```

<br>

## Three Dices

[Question_Link - 2480](https://www.acmicpc.net/problem/2480)

### Basic Code

```rust
use std::io;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop(); // trim()과 비슷한 작업을 함
    buf
}

fn split_num() -> Vec<i32> {
    let s = input();
    s.split(' ')
    .map(
        |x| x.parse().unwrap()
    ).collect()
}
fn main() {
    let dices = &mut split_num()[..];
    dices.sort();
    let mut same_cnt = 0;
    let mut same = 0;
    let max = dices[2];
    for p in 0..2 {
        if same_cnt == 2 { break; }
        for tmp in &dices[p+1..] {
            if dices[p] == *tmp { same_cnt+=1; same = dices[p]; }
        }
    }
    let reward = match same_cnt {
        0 => { max*100 },
        1 => { 1000+same*100 },
        2 => { 10000+same*1000 },
        _ => { 0 },
    };
    println!("{reward}");
}
```

### Improvement Code

```rust
use std::io;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop(); // trim()과 비슷한 작업을 함
    buf
}

fn main() {
    let s = input();

    // 1~6까지의 눈을 가진 주사위 각 한 개씩 생성하는 코드
    let mut dices = [0; 7];
    s.split(' ').for_each(|x| dices[x.parse::<usize>().unwrap()] += 1);
    
    let reward = match dices.iter().zip(0..7).max().unwrap() {
        (3, m) => 10000+m*1000,
        (2, m) => 1000+m*100,
        (_, m) => m*100,
    };
    print!("{reward}");
}
```

<br>

[trait_Iterator_method_for_each in Official Document](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each)

[method_zip in Official Document](https://doc.rust-lang.org/std/iter/fn.zip.html)