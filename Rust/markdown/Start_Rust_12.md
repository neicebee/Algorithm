# 🦀 Rust Day 12

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

<br>

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

<br>

#### **🤔 `Option<T>` 를 이용한 매칭**
- `Option<T>` 타입이 `Some(T)` 값을 가질 때 그 안에 저장된 T 값을 꺼내 사용할 수 있음
- `match` 표현식에 적용 가능함

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}\n{:?}", six, none);
}
// Result
// Some(6)
// None
```
- 변수에 값이 존재하면 그 값에 1을 더하고 값이 존재하지 않으면 `None` 리턴

<br>

#### **🤔 `match` 는 반드시 모든 경우를 처리해야 한다**
- **_러스트의 패턴 매칭은 완벽해야 함_**
  - 가능한 모든 경우를 처리해야 함
  - `Option<T>` 의 경우 러스트가 `None` 에 해당하는 경우를 명시적으로 처리하도록 강제함으로써 어떤 값이 널값임에도 불구하고 다른 값을 가지고 있을 것으로 잘못 생각하는 일을 방지함

<br>

#### **🤔 자리지정자 `_`**
- 러스트는 모든 경우를 다 처리하고 싶지 않을 때 사용할 수 있는 패턴도 제공함

```rust
fn main() {
    let number = 6u8;
    match number {
        0 => println!("{}", number),
        2 => println!("{}", number),
        4 => println!("{}", number),
        6 => println!("{}", number),
        _ => (),
    }
}
// Result
// 6
```
- 별도의 처리가 필요하지 않을 때는 `_` 자리지정자로 대체하면 됨
  - `_` 패턴은 모든 값에 일치함을 의미
  - `()` 은 단순한 유닛값
- `match` 표현식은 단 한가지 경우만 처리해야 할 때 사용하기에는 다소 장황함

<br>

### **3️⃣ `if let` 을 이용한 간결한 흐름 제어**
- `if let` 구문은 `if` 와 `let` 구문을 조합해서 여러 경우 중 한 가지만 처리하고, 나머지는 고려하고 싶지 않을 때 처리하는 문법

```rust
fn main() {
    let number = Some(3u8);
    match number {
        Some(3) => println!("Three!"),
        _ => (),
    }
}
// Result
// Three!
```
- `match` 표현식을 이용한 처리 방법

```rust
fn main() {
    let number = Some(2u8);
    
    if let Some(3) = number {
        println!("Three!");
    }
}
// Result
// 
```
- `if let` 문법을 이용한 처리 방법
- `if let` 문법은 하나의 패턴과 표현식을 등호 기호를 이용해 조합함
  - `match` 표현식과 같은 방식으로 동작함
- 구문에 지정한 패턴은 `match` 표현식의 첫 번째 패턴에 해당하며 표현식은 그 패턴이 일치할 때 실행될 표현식임
- `match` 표현식이 제공하는 모든 경우의 수를 검사하는 기능을 활용할 수는 없음
- **_`if let` 문법은 주어진 값에 대해 하나의 패턴만 검사하고 나머지 값은 무시하는 `match` 표현식을 더 쉽게 사용하기 위한 `문법적 편의장치(syntax sugar)` 라고 생각_**

```rust
fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}.", state),
        _ => {
            count += 1;
            println!("{}", count);
        },
    }
}
// Result
// 1
```
- `if let` 구문의 `else` 구문에 해당하는 `match` 표현식의 `_` 패턴

```rust
fn main() {
    let coin = Coin::Penny;
    let mut count = 0;
    
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}.", state);
    } else {
        count += 1;
        println!("{}", count);
    }
}
```
- `if let` 구문에 `else` 구문을 포함한 코드
- 프로그램의 로직을 표현하는 데 있어 `match` 표현식이 너무 장황하게 보인다면 `if let` 표현식도 제공된다는 점을 기억

<br>

## **Summary**
- 사용자 정의 타입 생성
  - 나열된 값 중 하나를 표현하는 `열거자`
- 타입 시스템이 제공하는 오류 방지 기능
  - 표준 라이브러리의 `Option<T>` 타입
- 열것값이 다른 데이터를 포함하고 있을 때 값 추출
  - `match` 표현식
  - `if let` 표현식