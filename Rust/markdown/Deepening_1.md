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

```