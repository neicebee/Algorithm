# 🦀 Rust Day 11

## **🏳️ Enums and Pattern Matching**

### **2️⃣ match 흐름 제어 연산자**
- `match` 표현식 : `흐름 제어 연산자(control flow operator)`
- 일련의 패턴과 값을 비교해 일치하는 패턴에 지정된 코드를 실행함
- 패턴은 리터럴, 변수 이름, 와일드카드를 비롯해 다양한 값으로 구성 가능
- **😎 장점**
  - 패턴에 대한 풍부한 표현력
  - 컴파일러가 모든 경우의 수가 처리되고 있는지 확인할 수 있음

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("{}", value_in_cents(coin));
}
// Result
// Lucky Penny!
// 1
```
- 열거자와 각 값에 해당하는 패턴을 `match` 표현식으로 작성함
- `match` 와 `if` 는 유사성을 띄지만 한 가지 큰 차이점이 있음
  - `if` 문의 표현식은 반드시 `bool` 을 리턴해야 하지만, `match` 연산자의 표현식은 어떤 타입의 값도 사용할 수 있음
- `match` 표현식은 실행 시점에 각 가지의 패턴과 결괏값을 순서대로 비교함
  - 패턴의 가지는 필요한 만큼 정의할 수 있음
  - 각 가지의 연관된 코드는 **_표현식_** 이며 일치하는 가지의 표현식을 실행한 **_결괏값_** 이 전체 `match` 표현식의 **_리턴값_** 이 됨
- 패턴 가지의 연관된 코드가 **_짧은 경우_** 에는 통상적으로 중괄호를 사용하지 않음
  - 패턴 가지에서 여러 줄의 코드를 실행하고자 한다면 중괄호 사용

#### **🤔 값을 바인딩하는 패턴**
- `match` 표현식의 가지 표현식은 패턴에 일치하는 값의 일부를 바인딩할 수 있음

```rust
#[derive(Debug)]
enum UsState {
    Alabama, Alaska, Arizona,
    Arkansas, California, Colorado,
    Florida, Kentucky, Ohio,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}.", state);
            25
        },
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Ohio);
    println!("{}", value_in_cents(coin));
}
// Result
// State quarter from Ohio.
// 25
```
- `Coin` 열거자의 열것값 `Quarter` 에 `UsState` 열거자를 추가함
- 패턴에 `state` 변수를 추가하여 `Coin::Quarter` 열것값과 비교함
  - `Coin::Quarter` 열것값이 일치하면 `state` 변수에는 열것값이 저장된 `UsState` 열거자가 바인딩됨

#### **🤔 `Option<T>` 를 이용한 매칭**