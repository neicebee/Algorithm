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