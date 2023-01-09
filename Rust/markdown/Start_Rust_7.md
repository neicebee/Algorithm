# 🦀 Rust Day 7

## **🏳️ Rust Example Script 1 - Temperature Converter**

### **1️⃣ Description**
- 화씨를 입력하면 섭씨로, 섭씨를 입력하면 화씨로 변환하는 프로그램 작성

### **2️⃣ how it works**
- 섭씨는 `C`, 화씨는 `F` 를 입력받는다. (대문자와 소문자 구별하지 않음)
- 온도를 입력받는다.
- 결과값을 변환 단위에 맞춰 출력한다. (결과가 소수점을 가질 때 해당 값에서 가장 가까운 정수로 출력)

### **3️⃣ Code**
```rust
use std::io;

// 온도 입력 함수
fn input_temperature() -> f64 {
    loop{
        println!("온도를 입력해주세요...");

        let mut tem = String::new();
        io::stdin().read_line(&mut tem)
            .expect("Unable to read input!");

        let tem: f64 = match tem.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error! You have to input number!");
                continue
            },
        };

        break tem;
    }
}

// 계산 함수
fn calculation(u: &str, t: f64) -> f64 {
    let mut result: f64 = 0.0;

    if u=="C"||u=="c" {
        result = (t*1.8)+32.0;
    } else if u=="F"||u=="f" {
        result = (t-32.0)/1.8;
    }   

    return result;
}

// 메인 함수
fn main() {
    loop{
        println!("단위 입력(C & F)...");

        // 입력한 값을 문자열로 저장하기 위한 변수 초기화
        let mut unit = String::new();

        // 사용자 입력을 위한 코드
        io::stdin().read_line(&mut unit)
            .expect("Unable to read input!");
        
        let unit: &str = unit.trim();

        if unit=="C"||unit=="c" {
            println!("Celsius");
            let result: f64 = calculation(unit, input_temperature()).round();
            println!("{}F", result);
            break;
        } else if unit=="F"||unit=="f" {
            println!("Fahrenheit");
            let result: f64 = calculation(unit, input_temperature()).round();
            println!("{}C", result);
            break;
        } else {
            println!("다시 입력하세요...");
            continue;
        }
    }
}
```

### **4️⃣ Result**
- C to F - 1
    ```rust
    단위 입력(C & F)...
    C // input
    Celsius // output
    온도를 입력해주세요...
    100 // input
    212F // output
    ```

- C to F - 2
    ```rust
    단위 입력(C & F)...
    C // input
    Celsius // output
    온도를 입력해주세요...
    37 // input
    99F // output

    // 37C는 98.6F
    ```

- F to C - 1
    ```rust
    단위 입력(C & F)...
    F // input
    Fahrenheit // output
    온도를 입력해주세요...
    212 // input
    100C // output
    ```

- F to C - 2
    ```rust
    단위 입력(C & F)...
    F // input
    Fahrenheit // output
    온도를 입력해주세요...
    100 // input
    38C // output

    // 100F는 37.777...C
    ```

**[유익한 글 - Rust의 String과 str](https://it-neicebee.tistory.com/148)**