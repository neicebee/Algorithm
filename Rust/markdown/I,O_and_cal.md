# 💻 Baekjoon I/O & Calculation Stage

## Sum

```rust
use std::io;

fn main(){

    let mut nums = String::new();
    io::stdin().read_line(&mut nums)
        .expect("Unable to read input!");
    
    let splitnums: Vec<&str> = nums.split(' ')
        .collect();

    let a = &splitnums[0];
    let b = &splitnums[1];

    let a: u32 = match a.trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };

    let b: u32 = match b.trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };

    println!("{}", a+b);
}
```

## Dif

```rust
use std::io;

fn main(){

    let mut nums = String::new();
    io::stdin().read_line(&mut nums)
        .expect("Unable to read input!");
    
    let splitnums: Vec<&str> = nums.split(' ')
        .collect();

    let a = &splitnums[0];
    let b = &splitnums[1];

    let a: i32 = match a.trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };

    let b: i32 = match b.trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };

    println!("{}", a-b);
}
```

## Mul

```rust
use std::io;

fn main(){

    let mut nums = String::new();
    io::stdin().read_line(&mut nums)
        .expect("Unable to read input!");
    
    let splitnums: Vec<&str> = nums.split(' ')
        .collect();

    let a = &splitnums[0];
    let b = &splitnums[1];

    let a: i32 = match a.trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };

    let b: i32 = match b.trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };

    println!("{}", a*b);
}
```

## Div

```rust
use std::io;

fn main(){

    let mut nums = String::new();
    io::stdin().read_line(&mut nums)
        .expect("Unable to read input!");
    
    let splitnums: Vec<&str> = nums.split(' ')
        .collect();

    let a = &splitnums[0];
    let b = &splitnums[1];

    let a: f64 = match a.trim().parse() {
        Ok(x) => x,
        Err(_) => 0.0,
    };

    let b: f64 = match b.trim().parse() {
        Ok(x) => x,
        Err(_) => 0.0,
    };

    println!("{}", a/b);
}
```

