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

## Test Score

[Question_Link - 9498](https://www.acmicpc.net/problem/9498)

### Basic Code

