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
- 구조체를 사용하지 않고 데이터를 열거자의 열것값에 직접 저장하여 표현함