# 🦀 Rust Day 11

## **🏳️ Enums and Pattern Matching**
- `열거자(enumerations, enums)` 는 사용 가능한 값만 나열한 타입을 정의할 때 사용
- 패턴 매칭은 열거자가 가진 여러 값에 대해 각기 다른 코드를 쉽게 실행할 수 있는 방법
- 러스트의 열거자는 _F#, OCaml, 하스켈(Haskell)_ 과 같은 함수형 언어의 `대수식(algebraic)` 데이터 타입에 더 가까움

### **1️⃣ 열거자 정의하기**
- IP 주소는 버전 4나 버전 6 형식의 주소지만 동시에 두 형식을 지원할 수 없음
- 이러한 IP 주소의 특징 덕분에 구조체보다는 열거자 데이터 구조를 적용하는 것이 적합함
  - 열거자에 나열한 값은 반드시 하나만 사용할 수 있음

```rust
enum IpAddrKind {
    V4,
    V6,
}
```
- IP 주소의 형식을 나타내는 `IPAddrKind` 열거자 정의 후 각 형식을 표현하기 위한 `V4` 와 `V6` 값 정의
  - 해당 값들을 열거자의 `열것값(variants)` 이라고 칭함
  - `IpAddrKind` 열거자는 이제 코드에서 사용할 수 잇는 하나의 타입으로 간주됨

#### **🤔 열거자의 값**

```rust
let ipv4_1 = IpAddrKind::V4;
let ipv6_1 = IpAddrKind::V6;
```
- 열거자의 각 값을 표현하는 인스턴스
- 열거자의 각 값은 식별자를 이용해 구분함
  - 식별자와 값은 `::` 문법으로 구분

```rust
fn route(ip_type: IpAddrKind) { }
```
- 열거자를 매개변수로 갖는 함수도 정의할 수 있음

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let switch = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{}\n{}", home.address, switch.address);
}
// Result
// 127.0.0.1
// ::1
```
- 구조체를 이용해 IP 주소의 종류와 데이터를 저장함
- `IpAddr` 구조체 정의 후 `IpAddrKind` 열거자 타입의 `kind` 필드, `String` 타입의 `address` 필드 정의
- 구조체를 이용해 IP 주소의 종류와 실제 주소를 하나로 묶었기 때문에 열거자의 값과 관련된 값을 하나로 처리할 수 있음

```rust
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(
        String::from("127.0.0.1")
    );
    let switch = IpAddrKind::V6(
        String::from("::1")
    );

    println!("{:?}\n{:?}", home, switch);
}
// Result
// V4("127.0.0.1")
// V6("::1")
```
- 데이터를 열거자의 열것값에 직접 저장하여 표현함
- `IpAddr` 열거자는 `V4` 와 `V6` 값을 정의하면서 연관된 값의 타입을 `String` 타입으로 명시
- 별도의 구조체를 선언할 필요가 없음

```rust
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddrKind::V4(
        127, 0, 0, 1
    );
    let switch = IpAddrKind::V6(
        String::from("::1")
    );

    println!("{:?}\n{:?}", home, switch);
}
// Result
// V4(127, 0, 0, 1)
// V6("::1")
```
- 열거자에 나열된 각각의 값을 서로 다른 타입과 다른 수의 연관 데이터를 보유할 수 있음
- `V4` 형식의 주소에 네 개의 `u8` 값을 저장, `V6` 형식의 주소는 그대로 `String` 타입
- 구조체로는 이런 데이터 구조를 갖출 수 없음
- 열거자의 값에는 _문자열, 숫자, 구조체 등 어떤 종류의 데이터_ 도 저장할 수 있음
  - _다른 열거자를 저장해도 무방함_
- 표준 라이브러리에 열거자가 정의되어 있다고 해도 표준 라이브러리에 정의한 타입을 코드의 범위로 가져오지 않는 한 아무런 문제 없이 같은 이름의 타입을 재정의할 수 있음
  
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}
```
- 개별 값을 각기 다른 타입으로 정의한 `Message` 열거자
  - `Quit` : 연관 데이터를 갖지 않음
  - `Move` : `익명 구조체(anonymous struct)` 포함
  - `Write` : 하나의 `String` 값 포함
  - `ChangeColor` : 세 개의 `i32` 값 포함
  - 각기 다른 구조체 타입을 정의하는 것과 유사함
    - **다른점 1.** `struct` keyword를 사용하지 않음
    - **다른점 2.** 열거자의 개별 값들은 모두 열거자 타입에 속함

```rust
struct QuitMsg; // 유닛 구조체
struct MoveMsg { 
    x: i32, 
    y: i32,
}
struct WriteMsg(String); // 튜플 구조체
struct ChangeColorMsg(i32, i32, i32); // 튜플 구조체
```
- `Message` 열거자와 같은 데이터를 저장하도록 정의한 구조체
- 다른 타입의 구조체를 정의하게 되면 여러 종류의 메시지를 매개변수로 전달받는 함수를 쉽게 정의할 수 없음

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let msg = Message::Write(
        String::from("Hello, world!")
    );

    msg.call();
}
// Result
// Write("Hello, world!")
```
- `impl` 블록을 이용하여 열거자의 메서드를 정의함
- `self` keyword로 메서드를 호출하는 열거자의 값을 가져올 수 있음

#### **🤔 `Option` 열거자를 `Null` 값 대신 사용할 때의 장점**
- `Option` 타입은 매우 다양한 곳에서 활용 가능
  - 어떤 값이 존재하거나 존재하지 않는, 아주 범용적인 시나리오에 적합하도록 디자인되었기 때문
  - 타입 시스템이 이런 타입을 제공한다는 것은 코드가 모든 경우의 수를 처리하고 있는지를 컴파일러가 확인할 수 있다는 것을 의미
- 러스트는 다른 언어가 가지고 있는 `널(null)` 이라는 값의 개념이 없음
  - `널값` : 아무런 값을 갖지 않는 경우를 의미
  - `널값` 을 지원하는 언어에서 변수는 항상 `널` 이거나 `널이 아닌(not-null)` 두 가지 상태 중 하나임
- `널값` 의 문제점은 `널값` 을 `널이 아닌 값` 처럼 사용하려고 하면 에러가 발생한다는 점임
- 러스트는 어떤 값의 존재 여부를 표현하는 열거자를 정의하고 있음
- **😎 `Option<T>`**

    ```rust
    // 표준 라이브러리에서 Option<T>의 정의
    enum Option<T> {
        Some(T),
        None,
    }
    ```
    - `Option<T>` 열거자는 `prelude` 에 포함되어 있기에 열것값도 함께 명시적으로 범위를 가져올 필요가 없음
      - `Option::` 접두어 없이 `Some` 이나 `None` 값 사용 가능
      - [prelude에 대한 공식 문서](https://doc.rust-lang.org/std/prelude/index.html)
    - 러스트의 `<T>` 문법은 `제네릭(generic)` 타입의 매개변수를 의미
      - `<T>` 매개변수 덕분에 `Option` 열거자의 `Some` 값은 어떤 타입의 데이터도 저장이 가능함
      - **Ex1.** `let num = Some(5);` $=>$ 숫자 타입
      - **Ex2.** `let msg = Some("Hello, world!");` $=>$ 문자열 타입
      - `Some` 대신 `None` 값을 이용하면 `Option<T>` 열거자 타입을 명시해야 함
      - **Ex1.** `let null_num: Option<i32> = None;`
    - `Option<T>` 와 `T` 는 다른 타입이기 때문에 컴파일러는 유효한 값이 명확히 존재할 때는 `Option<T>` 값을 사용하는 것을 허락하지 않음
      - `i8` 타입의 값을 갖고 있다면 이 값은 항상 유효한 값이라 가정하기 때문에 이 값을 사용하기 전에 널 검사 같은 것을 할 필요가 없음
      - `Option<i8>` 타입을 사용할 때만 이 변수에 값이 없을 가능성이 있기에 사용에 앞서 값이 없는 경우를 처리하려고 함
    - `T` 타입에 대한 작업 실행 전에 `Option<T>` 타입을 `T` 타입으로 변환해야 함
- 어떤 값이 `널값` 을 가질 수 있도록 하려면 `Option<T>` 타입 명시
- 값을 사용하고자 하면 이 값이 널인 경우를 반드시 명시적으로 처리
- `Option<T>` 가 아닌 다른 타입의 값을 사용하는 경우라면 이 값은 널이 아닐 것으로 생각해도 무방
- `Option<T>` 의 다양한 메서드 in Official Document
  - [https://doc.rust-lang.org/stable/std/option/enum.Option.html](https://doc.rust-lang.org/stable/std/option/enum.Option.html)
- 통상 `Option<T>` 값을 사용하려면 열거자에 나열된 개별 값을 처리할 코드를 작성해야 함
  - `match` 표현식을 사용해 쉽게 구현 가능
    - **(1).** `Some(T)` 값을 가진 경우만 실행 -> 열거자 안에 저장된 `T` 의 값에 접근할 수 있음
    - **(2).** `None` 값을 가진 경우에 실행 -> `T` 값에 접근 불가능