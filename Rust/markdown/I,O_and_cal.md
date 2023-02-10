# 💻 Baekjoon I/O & Calculation Stage

## Hello World

[Question_Link - 2557](https://www.acmicpc.net/problem/2557)

```rust
fn main() {
    println!("Hello World!");
}
```

<br>

## A+B

[Question_Link - 1000](https://www.acmicpc.net/problem/1000)

### Basic Code

```rust
use std::io;

fn main() {
    let mut numbers = String::new();
    let split_nums: Vec<&str> = match io::stdin().read_line(&mut numbers) {
        Ok(_n) => numbers.split(' ').collect(),
        Err(_) => vec!["Error"],
    };
    let a: u32 = match split_nums[0].trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };
    let b: u32 = match split_nums[1].trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };
    println!("{}", a+b);
}
```

### Improvement Code

```rust
use std::io::*;

fn main() {
    let mut numbers = [0; 3];
    stdin().read(&mut numbers)
        .expect("No input");
    println!("{}", numbers[0]+numbers[2]-96);
}
```
<br>

[struct_Stdin in Official Document](https://doc.rust-lang.org/std/io/struct.Stdin.html)

[function_stdin in Official Document](https://doc.rust-lang.org/std/io/fn.stdin.html)

[trait_Read in Official Document](https://doc.rust-lang.org/std/io/trait.Read.html)

<br>

## A-B

[Question_Link - 1001](https://www.acmicpc.net/problem/1001)

### Basic Code

```rust
use std::io;

fn main() {
    let mut numbers = String::new();
    let split_nums: Vec<&str> = match io::stdin().read_line(&mut numbers) {
        Ok(_n) => numbers.split(' ').collect(),
        Err(_) => vec!["Error"],
    };
    let a: i32 = match split_nums[0].trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };
    let b: i32 = match split_nums[1].trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };
    println!("{}", a-b);
}
```

### Improvement Code

```rust
use std::io;

fn main() {
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers)
        .expect("No input");
    let split_nums: Vec<i8> = numbers.split(' ')
        .map(
            |x| x.trim().parse().expect("Error")
        ).collect();
    println!("{}", split_nums[0]-split_nums[1]);
}
```

<br>

[struct_Map in Official Document](https://doc.rust-lang.org/std/iter/struct.Map.html)

[trait_Iterator_method_collect in Official Document](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)

<br>

## A$\times$B

[Question_Link - 10998](https://www.acmicpc.net/problem/10998)

### Basic Code

```rust
use std::io;

fn main() {
    let mut numbers = String::new();
    let split_nums: Vec<&str> = match io::stdin().read_line(&mut numbers) {
        Ok(_n) => numbers.split(' ').collect(),
        Err(_) => vec!["Error"],
    };
    let a: u32 = match split_nums[0].trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };
    let b: u32 = match split_nums[1].trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };
    println!("{}", a*b);
}
```

### Improvement Code

```rust
use std::io;

fn main() {
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers)
        .expect("No input");
    println!(
        "{:?}",
        numbers.split(' ')
        .map(
            |x| x.trim().parse().expect("error")
        ).fold(1, |a, b: i32| a*b)
    );
}
```

<br>

[trait_Iterator_method_fold in Official Document](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)

<br>

## A/B

[Question_Link - 1008](https://www.acmicpc.net/problem/1008)

### Basic Code

```rust
use std::io;

fn main() {
    let mut numbers = String::new();
    let split_nums: Vec<&str> = match io::stdin().read_line(&mut numbers) {
        Ok(_n) => numbers.split(' ').collect(),
        Err(_) => vec!["Error"],
    };
    let a: f64 = match split_nums[0].trim().parse() {
        Ok(x) => x,
        Err(_) => 0.0,
    };
    let b: f64 = match split_nums[1].trim().parse() {
        Ok(x) => x,
        Err(_) => 0.0,
    };
    println!("{}", a/b);
}
```

### Improvement Code

```rust
use std::io;

fn main() {
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers)
        .expect("No input");
    let split_nums: Vec<f64> = numbers.split(' ')
        .map(
            |x| x.trim().parse().expect("error")
        ).collect();
    println!("{}", split_nums[0]/split_nums[1]);
}
```

<br>

## Four Arithmetic Operations

[Question_Link - 10869](https://www.acmicpc.net/problem/10869)

```rust
use std::io;

fn main() {
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers)
        .expect("No Input");
    let a: Vec<i32> = numbers.split(' ')
        .map(
            |x| x.trim().parse().expect("Error")
        ).collect();
    
    println!(
        "{}\n{}\n{}\n{}\n{}",
        a[0]+a[1],
        a[0]-a[1],
        a[0]*a[1],
        a[0]/a[1],
        a[0]%a[1]
    );
}
```

<br>

## ??!

[Question_Link - 10926](https://www.acmicpc.net/problem/10926)

### Basic Code

```rust
use std::io;

fn main() {
    let mut id = String::new();
    io::stdin().read_line(&mut id)
        .expect("Error");
    id = id.trim().to_string() + "??!";
    println!("{id}");
}
```

### Improvement Code

```rust
use std::io;

fn main() {
    let mut id = String::new();
    io::stdin().read_line(&mut id)
        .expect("Error");
    println!("{}??!", id.trim());
}
```

<br>

## Buddhism Year

[Question_Link - 18108](https://www.acmicpc.net/problem/18108)

- 서기 연도와 불기 연도의 차이는 `543` 년임

## Basic Code

```rust
use std::io;

fn main() {
    let mut year = String::new();
    io::stdin().read_line(&mut year)
        .expect("No Input");
    let bud_year: u16 = year.trim().parse()
        .expect("Error");
    println!("{}", bud_year-543);
}
```

### Improvement Code

```rust
use std::io;

fn main() {
    let mut year = String::new();
    io::stdin().read_line(&mut year)
        .expect("No Input");
    println!(
        "{}",
        year.trim().parse::<u16>().expect("Error") - 543
    );
}
```

<br>

## Chess Pieces

[Question_Link - 3003](https://www.acmicpc.net/problem/3003)

### Basic Code

```rust
use std::io;
const CHESS_SET: [i32; 6] = [1, 1, 2, 2, 2, 8];
fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a)
        .expect("Error");
    let pieces: Vec<i32> = a.split(' ')
        .map(
            |x| x.trim().parse().expect("error")
        ).collect();
    for i in 0..6 {
        print!("{} ", CHESS_SET[i]-pieces[i]);
    }
}
```

<br>

## Modulo

[Question_Link - 10430](https://www.acmicpc.net/problem/10430)

### Basic Code

```rust
use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a)
        .expect("Error");
    let nums: Vec<u32> = a.split(' ')
        .map(
            |x| x.trim().parse().expect("error")
        ).collect();
    println!(
        "{}\n{}\n{}\n{}",
        (nums[0]+nums[1])%nums[2],
        ((nums[0]%nums[2])+(nums[1]%nums[2]))%nums[2],
        (nums[0]*nums[1])%nums[2],
        ((nums[0]%nums[2])*(nums[1]%nums[2]))%nums[2]
    );
}
```

### Additional Code

```rust
use std::io;

fn main() {
    let mut z = String::new();
    io::stdin().read_line(&mut z)
        .expect("Error");
    match &z.split(' ')
        .map(
            |x| x.trim().parse().expect("Error")
        ).collect::<Vec<u32>>()[..3] {
        &[a, b, c] => {
            println!(
                "{}\n{}\n{}\n{}",
                (a+b)%c, ((a%c)+(b%c))%c,
                (a*b)%c, ((a%c)*(b%c))%c
            );
        },
        _ => (),
    }
}
```

<br>

## Multiplication

[Question_Link - 2588](https://www.acmicpc.net/problem/2588)

### 