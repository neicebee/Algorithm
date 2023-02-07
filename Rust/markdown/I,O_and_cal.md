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

[Trait_Read in Official Document](https://doc.rust-lang.org/std/io/trait.Read.html)

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

<br>

## A$\times$B

[Question_Link - 10998](https://www.acmicpc.net/problem/10998)

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

<br>

## A/B

[Question_Link - 1008](https://www.acmicpc.net/problem/1008)

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

<br>

## Four Arithmetic Operations

[Question_Link - 10869](https://www.acmicpc.net/problem/10869)

```rust

```