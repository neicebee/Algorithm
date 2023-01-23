# 🦀 Rust Day 8

## **🏳️ Rust Example Script 2 - Nth in Fibonacci Sequence**

### **1️⃣ Description**
- 조건 `0<=n<=45` 에 부합하는 정수 n을 입력받아 n번째 피보나치 수열을 출력하는 프로그램 작성

### **2️⃣ how it works**
- 조건 `0<=n<=45` 에 부합하는 정수 n을 입력받는다.
- n번째 피보나치 수열을 출력한다.

### **3️⃣ Code**
```rust
use std::io;

fn simple_fibo(n: u32) -> u64 {
    let mut a = 0;
    let mut b = 1;

    if n <= 1 {
        return n.into();
    }

    for _ in 0..n-1 {
        let tmp = a;
        a = b;
        b = tmp+b;
    }
    return b;
}

fn array_fibo(n: u32) -> usize {
    let mut fibo_arr: [usize; 2] = [0, 1];
    let n: usize = n.try_into().unwrap();

    for i in fibo_arr.len()..n+1 {
        fibo_arr[i%2] = fibo_arr[(n-1)%2] + fibo_arr[(n-2)%2];
    }

    return fibo_arr[n%2]
}

fn recursive_fibo(n: u32) -> u64 {
    if n <= 1 {
        return n.into();
    } else {
        return recursive_fibo(n-1) + recursive_fibo(n-2);
    }
}

fn main() {
    loop {
        let mut number = String::new();

        io::stdin().read_line(&mut number)
            .expect("Unable to read input!");
        
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error! You have to input number!");
                continue
            },
        };

        println!("Simple Fibonacci Sequence: {}", simple_fibo(number));
        println!("Recursive Fibonacci Sequence: {}", recursive_fibo(number));
        println!("Array Fibonacci Sequence: {}", array_fibo(number));
        break;
    }
}
```

### **4️⃣ Result**
- n: 15
    ```rust
    15 // input
    Simple Fibonacci Sequence: 610 // output
    Recursive Fibonacci Sequence: 610 // output
    Array Fibonacci Sequence: 610 // output
    ```
- n: 44
    ```rust
    44 // input
    Simple Fibonacci Sequence: 701408733 // output
    Recursive Fibonacci Sequence: 701408733 // output
    Array Fibonacci Sequence: 701408733 // output
    ```

### **5️⃣ Reference**
- Febonacci Sequence in Python
  - [https://it-neicebee.tistory.com/150](https://it-neicebee.tistory.com/150)
  - **_이 글의 `재귀적 동적 계획법` 및 `행렬 연산` 에 대한 Rust 구현은 필자의 Rust 숙련도 부족으로 구현하지 못했음. 앞으로 더 공부해서 해당 방법을 Rust로 구현해보는 글을 업로드할 예정임._**
- Rust official Document about `core::convert::Into`
  - [https://doc.rust-lang.org/beta/core/convert/trait.Into.html](https://doc.rust-lang.org/beta/core/convert/trait.Into.html)
- Rust official Document about `core::convert::TryInto`
  - [https://doc.rust-lang.org/beta/core/convert/trait.TryInto.html](https://doc.rust-lang.org/beta/core/convert/trait.TryInto.html)