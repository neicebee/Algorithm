# 🦀 Rust Day 19

## **🏳️ Generic Types, Traits, and Lifetimes**
- 러스트의 제네릭(Generic)은 구체적인 타입(concrete type)이나 다른 속성으로 대체할 수 있는 추상화된 타입을 활용함
- 구체적인 타입 대신 제네릭 타입을 매개변수로 선언할 수 있음
  - `Option<T>`, `Vec<T>`, `HashMap<K, V>`, `Result<T, E>` 모두 제네릭 타입임
- 수명은 제네릭의 일종으로 참조가 서로 어떻게 관련이 있는지에 대한 정보를 컴파일러에 제공함

<br>

### **1️⃣ 함수로부터 중복 제거하기**

```rust
fn main() {
    let num_list_1 = vec![34, 56, 77, 25, 100, 54];
    let num_list_2 = vec![102, 24, 6000, 89, 54, 2, 43, 8];

    let mut max = num_list_1[0];

    for number in num_list_1 {
        if number > max {
            max = number;
        }
    }

    println!("num_list_1's max: {max}");

    max = num_list_2[0];

    for number in num_list_2 {
        if number > max {
            max = number;
        }
    }

    println!("num_list_2's max: {max}");
}
// Result
// num_list_1's max: 100
// num_list_2's max: 6000 
```
- 두 개의 숫자 리스트에서 가장 큰 값을 출력하는 코드
- 코드가 중복되는 부분이 있음

```rust
fn get_max(num_list: &[i32]) -> i32 {
    let mut max = num_list[0];
    
    for number in num_list {
        if number > &max {
            max = *number;
        }
    }

    max
}

fn main() {
    let num_list_1 = vec![34, 56, 77, 25, 100, 54];
    let num_list_2 = vec![102, 24, 6000, 89, 54, 2, 43, 8];

    println!("num_list_1's max: {}", get_max(&num_list_1));
    println!("num_list_2's max: {}", get_max(&num_list_2));
}
// Result
// num_list_1's max: 100
// num_list_2's max: 6000
```
- 중복된 부분을 함수화시킨 코드
- 함수의 매개변수는 `i32` 값의 슬라이스라면 어떤 타입이든 전달할 수 있음

<br>

### **2️⃣ 제네릭 데이터 타입**
- 제네릭은 여러 구체화된 타입을 사용할 수 있는 함수 시그니처나 구조체 같은 아이템을 정의할 때 사용함

#### **🤔 함수 정의에서 사용하기**

```rust
fn get_i32_max(num_list: &[i32]) -> i32 {
    let mut max = num_list[0];
    
    for number in num_list {
        if number > &max {
            max = *number;
        }
    }

    max
}

fn get_char_max(char_list: &[char]) -> char {
    let mut max = char_list[0];

    for char in char_list {
        if char > &max {
            max = *char;
        }
    }

    max
}

fn main() {
    let num_list = vec![34, 56, 77, 25, 100, 54];
    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("num_list's max: {}", get_i32_max(&num_list));
    println!("char_list's max: {}", get_char_max(&char_list));
}
// Result
// num_list's max: 100
// char_list's max: y
```
- 매개변수와 리턴 타입만 다르고 동작은 같은 두 중복 함수 코드
- 두 함수의 본문은 완전히 같기 때문에 이 중복을 제거하려면 제네릭 타입을 이용해 하나의 함수로 정의해야 함
- 함수의 본문에 매개변수가 필요할 시 매개변수의 이름을 시그니처에 선언해서 컴파일러가 그 이름의 의미를 판단할 수 있도록 함

```rust
fn get_max<T>(list: &[T]) -> T {
```
- 제네릭 `get_max` 함수를 정의할 시 함수의 이름과 매개변수 목록 사이의 `꺾쇠괄호(<>)` 에 타입 이름을 명시하면 됨
- `get_max` 함수 정의
  - **_함수 `get_max` 는 어떤 타입 `T` 를 일반화한 함수_** 
- 위의 제네릭 `get_max` 함수를 구현해서 작성한 코드는 컴파일 시 에러가 발생함
  - 에러 메시지에서는 `std::cmp::PartialOrd` 트레이트를 언급함
  - `get_max` 함수의 본문이 모든 타입 `T` 에 대해 동작하지 않는다는 것을 설명함
  - 함수 본문에서 타입 `T` 의 값을 비교하므로 이 값은 반드시 정렬 가능해야 함

#### **🤔 구조체 정의에서 사용하기**

```rust
struct Point<T> {
    x: T,
    y: T,
}
```
- x와 y 좌표의 값을 어떤 타입으로도 저장할 수 있는 `Point<T>` 구조체 선언
- 하나의 제네릭 타입만을 사용하므로 타입 `T` 를 일반화한 구조체
- 필드의 값은 모두 같은 타입이어야 함

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```
- 다중 제네릭 타입 매개변수를 사용하여 필드의 값들을 각각 다른 타입의 제네릭 데이터 타입으로 선언 가능
- 제네릭 타입이 너무 많이 필요해진다는 것은 코드를 더 작은 부분으로 재구성해야 한다는 것을 뜻함

#### **🤔 열거자 정의에서 사용하기**

```rust
enum Option<T> {
    Some(T),
    None,
}
```
- `Option<T>` 열거자 정의
  - 타입 `T` 를 일반화한 열거자
  - `Option<T>` 열거자를 이용하면 선택적인 값의 개념 추상화 가능
  - `Option<T>` 는 제네릭 열거자이므로 선택적인 값의 타입과 무관하게 추상화된 타입을 사용할 수 있음

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
- `Result<T, E>` 열거자 정의
  - 두 개의 타입 `T` 와 `E` 를 일반화한 타입

#### **🤔 메서드 정의에서 사용하기**

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point{
        x: 2,
        y: 5,
    };

    println!("{}", p.get_x());
}
// Result
// 2
```
- `Point<T>` 구조체의 필드 x의 값에 대한 참조를 리턴하는 메서드 정의
- `impl` keyword 다음에 타입 `T` 를 지정하면 러스트는 `Point` 구조체의 꺾쇠 괄호 안에 지정된 타입이 구체화된 타입이 아닌 제네릭 타입이라는 점을 인식함

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f64> {
    fn dis_from_ori(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point{
        x: 3.4,
        y: 8.9,
    };

    println!("{}", p.dis_from_ori());
}
// Result
// 9.527329111561121
```
- 특정 타입의 인스턴스에만 적용할 메서드 구현
- `Point<f64>` 타입은 해당 메서드를 사용할 수 있지만 다른 `Point<T>` 인스턴스는 해당 메서드를 사용할 수 없음
- `dis_from_ori` 메서드는 기준 좌표(0.0, 0.0)으로부터 지정된 좌표까지의 거리가 얼마나 먼지 계산하는 메서드임
  - 수학 연산에 사용하는 메서드는 부동 소수점 타입에만 적용할 수 있는 메서드임

```rust
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn struct_mix<V, W> (self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 4.5, y: 7.7 };
    let p2 = Point { x: "Hello", y: 'r'};

    println!("{:?}", p1.struct_mix(p2));
}
// Result
// Point { x: 4.5, y: 'r' }
```
- 구조체의 정의와는 다른 제네릭 타입을 사용하는 메서드 구현
- `impl` 블록과 메서드 정의에 각각 다른 제네릭 매개변수를 사용할 수 있음
  - 제네릭 매개변수 `T` 와 `U` 는 구조체의 정의에 따라 `impl` 블록 다음에 지정되어 있음
  - 제네릭 매개변수 `V` 와 `W` 는 메서드에만 관련이 있으므로 메서드에 정의함
- 해당 코드에서 p1과 p2는 메서드 호출 시 해당 메서드에 소유권이 넘어가므로 메서드 작업이 끝난 후에는 유효하지 않음

#### **🤔 제네릭의 성능**
- 러스트가 제네릭을 구현하는 방식은 제네릭 타입을 사용한다고 해서 구체화된 타입을 사용할 때보다 성능이 떨어지지 않음
  - 컴파일 시점에 제네릭을 사용하는 코드를 `단일화(Monomorphzation)` 하기 때문
  - `단일화` : 컴파일 시점에 제네릭 코드를 실제로 사용하는 구체화된 타입으로 변환하는 과정
  - 컴파일러는 제네릭 코드가 호출되는 부분을 모두 찾아 제네릭 코드의 호출에 사용된 구체화된 타입을 사용하는 코드를 생성함

```rust
fn main() {
    let integer = Some(5);
    let float = Some(8.9);
}
```
- 우리가 일반적으로 사용하는 코드

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(8.9);
}
```
- 단일화된 버전의 코드
- 제네릭 `Option<T>` 는 컴파일러가 생성한 특정 정의로 교체됨

<br>

### **3️⃣ 트레이트: 공유 가능한 행위를 정의하는 방법**