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
- `트레이트(trait)` : 러스트 컴파일러에게 특정 타입이 어떤 기능을 실행할 수 있으며, 어떤 타입과 이 기능을 공유할 수 있는지를 알려주는 방법
  - 공유 가능한 행위를 추상화된 방식으로 정의하는 방법
  - 다른 언어에서 `인터페이스(interface)` 기능과 유사함

#### **🤔 트레이트 선언하기**
- 트레이트 정의는 어떤 목적을 이루는 데 필요한 일련의 행위를 정의하고 여러 타입에 적용할 메서드 시그니처를 그룹화하는 방법
- `NewArticle` 구조체는 `story` 필드에 특정 지역의 뉴스 콘텐츠를 저장하며, `Tweet` 구조체는 최대 280글자의 텍스트와 해당 트윗이 새 트윗인지, 리트윗된 트윗인지, 아니면 다른 트윗의 댓글인지를 가리키는 메타데이터를 포함
- 해당 구조체들의 인스턴스에 저장된 데이터를 요약해서 보여주는 라이브러리 구현

```rust
// src/lib.rs
pub trait Summary {
    fn summarize(&self) -> String;
}
```
- `trait` keyword를 이용한 트레이트 정의와 메서드 시그니처 정의
- 메서드 시그니처는 각각 별개의 줄에 선언하며 각 줄은 세미콜론으로 끝나야 함
  - 해당 트레이트를 구현하는 각 타입은 반드시 이 메서드의 본문에 자신의 행위를 구현해야 함

#### **🤔 타입에 트레이트 구현하기**

```rust
// src/lib.rs
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {}, {}", 
            self.headline, self.author, self.location
        )
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "{}: {}", 
            self.username, self.content
        )
    }
}
```
- `Summary` 트레이트를 이용해 필요한 행위 정의
- 타입에 트레이트를 구현하는 것은 보통의 메서드 구현과 유사함
- `impl` 블록에서 메서드 시그니처를 추가하고 해당 타입에 대해 수행해야 할 특정 동작을 구현

```rust
// src/main.rs
use trait_example::*;

fn main() {
    let tweet = Tweet {
        username: String::from("f1r3_r41n"),
        content: String::from("트레이트 구현 단계 ing.."),
        reply: false,
        retweet: false,
    };

    println!("새 트윗 1개: {}", tweet.summarize());
}
// Result
// 새 트윗 1개: f1r3_r41n: 트레이트 구현 단계 ing..
```
- `NewsArticle` 타입, `Tweet` 타입, `Summary` 트레이트는 `lib.rs` 파일 하나에 모두 정의했으므로 모두 같은 범위에 존재함
- 만약 별개의 라이브러리 범위에 정의된 구조체에 `Summary` 트레이트를 구현하고자 함
  - 전체 경로를 명시하여 자신이 정의한 타입에 `Summary` 트레이트 구현
  - 때문에 공개 트레이트로 선언하였음
- 트레이트 구현 시 한 가지 제약
  - 트레이트나 트레이트를 구현할 타입이 현재 크레이트의 로컬 타입이어야 함
    - `Display` 같은 표준 라이브러리의 트레이트를 자신의 크레이트 일부인 `Tweet` 타입에 구현할 수 있는 이유는 `Tweet` 타입이 자신의 크레이트의 로컬 타입이기 때문
    - `Summary` 트레이트는 자신의 크레이트의 로컬 타입이므로 `Vec<T>` 타입에 `Summary` 트레이트를 구현할 수 있음
  - 외부 타입에 외부 트레이트를 구현할 수 없음
    - `Display` 와 `Vec<T>` 는 모두 표준 라이브러리에 정의된 타입이며 자신의 크레이트의 로컬 타입이 아니기 때문에 자신의 크레이트 안에서 `Vec<T>` 타입에 `Display` 트레이트를 구현할 수 없음
  - `통일성(coherence)` 혹은 `고아 규칙(orphan rule)` 이라고 칭함

#### **🤔 기본 구현**
- 트레이트에 선언된 모든 메서드를 구현하도록 요구하는 것보다 일부 혹은 전체 메서드의 기본 동작을 구현해 주면 특정 타입에 구현할 때 메서드의 기본 구현을 그대로 사용하거나 재정의할 수 있음

```rust
// src/lib.rs
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(계속 읽기)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle { }

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "{}: {}", 
            self.username, self.content
        )
    }
}
```
- `Summary` 트레이트 정의 블록에서 `summarize` 메서드의 기본 구현을 제공
- `NewsArticle` 구조체에서 `summarize` 메서드를 정의하지 않거나 재정의하지 않으면 기본 구현이 제공됨

```rust
// src/main.rs
use trait_example::*;

fn main() {
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("korea"),
        author: String::from("f1r3_r41n"),
        content: String::from("text..."),
    };

    println!("새로운 기사: {}", article.summarize());
}
// Result
// 새로운 기사: (계속 읽기)
```
- `Tweet` 구조체의 `summarize` 메서드는 재정의된 구현이 제공됨
- 기본 구현을 재정의하기 위한 문법은 기본 구현을 제공하지 않는 트레이트의 메서드를 구현하는 문법과 같음

```rust
// src/lib.rs
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("{}님의 기사 더 읽기", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```
- 기본 구현은 같은 트레이트의 다른 메서드도 호출 가능
  - 다른 메서드가 기본 구현을 제공하지 않아도 가능

```rust
// src/main.rs
use trait_example::*;

fn main() {
    let tweet = Tweet {
        username: String::from("f1r3_r41n"),
        content: String::from("트레이트 구현 단계 ing.."),
        reply: false,
        retweet: false,
    };

    println!("새 트윗 1개: {}", tweet.summarize());
}
// Result
// 새 트윗 1개: @f1r3_r41n님의 기사 더 읽기
```
- 같은 메서드를 재정의하면서 기본 구현 코드를 호출할 수는 없음

#### **🤔 트레이트 매개변수**

```rust
// src/lib.rs
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {}, {}",
            self.headline, self.author, self.location
        )
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "{}: {}",
            self.username, self.content
        )
    }
}

pub fn notify(item: impl Summary) {
    println!("속보! {}", item.summarize());
}

// src/main.rs
use trait_example::*;

fn main() {
    let article = NewsArticle {
        headline: String::from("headline"),
        location: String::from("korea"),
        author: String::from("f1r3_r41n"),
        content: String::from("text..."),
    };

    println!("새로운 기사: {}", article.summarize());
    notify(article);
}
// Result
// 새로운 기사: headline, by f1r3_r41n, korea
// 속보! headline, by f1r3_r41n, korea
```
- `impl Trait` 문법 : 지정된 트레이트를 구현하는 모든 타입을 인수로 전달할 수 있음
  - 비교적 간단한 경우에는 잘 동작
  - 트레이트 경계라고 부르는 더 긴 문법을 간단히 표현하기 위한 편의 장치

<br>

**(1)** 트레이트 경계 문법

```rust
pub fn notify<T: Summary>(item: T) {
    println!("속보! {}", item.summarize());
}
```
- 트레이트 경계 문법 : 제네릭 타입 매개변수를 선언하는 꺾쇠괄호 안에 콜론을 이용해 지정
  - 서로 다른 타입인 두 개의 매개변수 선언 : `impl Trait` 문법
  - 같은 타입인 두 개의 매개변수 선언 : 트레이트 경계 문법

<br>

**(2)** + 문법으로 여러 트레이트 경계 정의하기

```rust
pub fn notify(item: impl Summary + Display) {
```
- `impl Trait` 문법
- `notify` 함수에서 `item` 매개변수 값에 출력 형식을 적용하는 동시에 `summarize` 메서드를 호출하고자 할 때 `+` 문법으로 트레이트 구현 명시

```rust
pub fn notify<T: Summary + Display>(item: T) {
```
- 트레이트 경계 문법

**(3)** where 절을 이용해 트레이트 경계 정리하기
- 여러 개의 제네릭 타입 매개변수를 갖는 함수에는 함수 이름과 매개변수 목록 사이에 너무 많은 트레이트 경계를 나열하게 되어 함수 시그니처를 읽기 어려워짐

```rust
fn function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
```
- 여러 개의 제네릭 타입 매개변수와 트레이트 경계

```rust
fn function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone, U: Clone + Debug {
```
- `where` 절을 이용해 간결하게 정리한 코드

<br>

#### **🤔 트레이트를 구현하는 값 리턴하기**

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("f1r3_r41n"),
        content: String::from("트레이트 구현 단계 ing.."),
        reply: false,
        retweet: false,
    }
}
```
- 리턴 타입에 `impl Summary` 를 지정했기에 `Summary` 트레이트를 구현하는 어떤 타입도 리턴할 수 있음
- 이 함수를 호출하는 코드는 실제 리턴 타입을 알지 못함
- `impl Trait` 문법은 하나의 타입을 리턴하는 경우에만 사용할 수 있음
  - 컴파일러가 `impl Trait` 문법을 구현하는 방법의 제약 때문

#### **🤔 트레이트 경계를 이용해 largest 함수 작성하기**

```rust
fn get_max<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];

    for &i in list.iter() {
        if i > max {
            max = i;
        }
    }

    max
}

fn main() {
    let num_list = vec![34, 56, 77, 25, 100, 54];
    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("max: {}", get_max(&num_list));
    println!("max: {}", get_max(&char_list));
}
// Result
// max: 100
// max: y
```
- `>` 연산자는 표준 라이브러리의 `std::cmp::PartialOrd` 트레이트의 기본 메서드로 정의되어 있음
  - `T` 타입에 `PartialOrd` 트레이트 경계를 지정하여 함수가 실제로 비교할 수 있는 타입의 슬라이스만을 처리할 수 있게 함
- 크기가 이미 정해진 스칼라 값은 스택에 저장되므로 `Copy` 트레이트를 구현하고 있으나 제네릭 변수에 `Copy` 트레이트를 구현하지 않는 타입의 값이 전달될 가능성이 생김
  - `T` 타입에 `Copy` 트레이트 경계를 지정하여 `Copy` 트레이트를 구현하는 타입에 대해서만 호출할 수 있게 함
- `Copy` 대신 `Clone` 트레이트를 사용하면 함수가 소유권을 가질 때 슬라이스의 각 값을 복제함
  - `clone` 함수를 사용한다는 것은 `String` 처럼 힙 데이터를 사용하는 타입은 더 많은 힙 메모리를 할당하며, 많은 양의 데이터를 처리할 때 속도가 떨어질 수 있음
- 슬라이스에 저장된 `T` 타입의 참조를 리턴하는 것
  - `Clone` 혹은 `Copy` 트레이트 경계를 선언할 필요 없으며 힙 메모리 할당도 필요하지 않음

<br>

**(1)** 트레이트 경계를 이용해 조건에 따라 메서드 구현하기

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("가장 큰 멤버는 x = {}", self.x);
        } else {
            println!("가장 큰 멤버는 y = {}", self.y);
        }
    }
}

fn main() {
    let p = Pair::new(2, 4);
    p.cmp_display();
}
// Result
// 가장 큰 멤버는 y = 4
```
- 제네릭 타입 매개변수를 사용하는 `impl` 블록에 트레이트 경계 사용 시 타입이 특정 트레이트를 구현하는지에 따라 메서드를 구현할 수 있음

```rust
impl<Display> ToString for T {
    // -- 생략 --
}

fn main() {
    let s = 3.to_string();
}
```
- `덮개 구현(blanket implementations)` : 타입에 트레이트 경계를 만족하는 트레이트를 구현하는 것
  - 러스트의 표준 라이브러리에서 매우 빈번하게 사용
  - 표준 라이브러리는 `Display` 트레이트를 구현하는 타입에는 `ToString` 트레이트도 함께 구현함
    - 표준 라이브러리의 덮개 구현 기법으로 인해 `Display` 를 구현하는 모든 타입에 대해 `ToString` 트레이트가 정의한 `to_string` 메서드 호출 가능

<br>

### **4️⃣ 수명을 이용해 참조 유효성 검사하기**
- `수명(lifetimes)` : 참조가 유효한 범위
  - 암묵적이며, 타입이 대부분 `추론(inferred)` 에 의해 결정되는 것처럼 수명 역시 추론을 토대로 동작
  - 수명이 달라질 수 있을 때 수명 애노테이션 추가

#### **🤔 수명을 이용해 죽은 참조의 발생 방지하기**

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}
// Result
// error[E0597]: `x` does not live long enough
```
- 수명의 주요 목적은 **_죽은 참조가 발생하는 것을 방지하는 것_**
  - 죽은 참조: 프로그램이 원래 의도했던 데이터와는 다른 데이터를 참조하게 될 때를 뜻함
- 변수 x는 안쪽의 범위를 벗어나는 순간 더는 유효하지 않기에 수명이 충분치 않다는 오류 발생
- 러스트 컴파일러는 `대여 검사기` 를 탑재하여 코드가 더 이상 유효하지 않은지 알 수 있음

<br>

#### **🤔 대여 검사기**
- `대여 검사기(borrow checker)` : 대여한 값이 현재 범위 내에서 유효한지 검사

```rust
fn main() {
    let x = 5;
    let r = &x;
    println!("r: {r}");
}
```
- 변수 x를 변수 r보다 수명을 늘려 죽은 참조 문제를 해결한 코드
  - 변수 x가 유효한 동안 변수 r이 참조하는 메모리가 항상 유효함

<br>

#### **🤔 함수의 제네릭 수명**

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let msg1 = String::from("abcd");
    let msg2 = "xyz";
    println!("longest: {}", longest(msg1.as_str(), msg2));
}
// Result
// error[E0106]: missing lifetime specifier
```
- 러스트 입장에서 리턴값이 변수 x와 y 중 어떤 것이 리턴될 지 알 수 없으므로 리턴 타입에 제네릭 수명 매개변수를 지정해야 함
- 함수에 전달되는 참조의 실제 수명도 알 수 없으므로 범위를 이용해 리턴하려는 참조가 항상 유효한지를 확인할 수 없음

<br>

#### **🤔 수명 애노테이션 문법**
- 수명 애노테이션은 참조의 유효 기간을 변경하지는 않으며 개별 참조의 수명에 영향을 주지 않으면서 여러 참조 간 수명의 관계를 서술할 수 있음
- 제네릭 수명 매개변수를 지정하면 어떤 수명의 참조도 전달할 수 있음

```rust
&i32            // 참조
&'a i32         // 명시적인 수명을 가진 참조
&'a mut i32     // 명시적인 수명을 가진 가변 참조
```
- 수명 애노테이션 문법
- 수명 애노테이션은 그 자체로 많은 의미를 갖지 않음
  - 러스트에게 제네릭 수명 매개변수가 지정된 각 참조가 서로 어떤 관계인지 명시할 뿐

<br>

#### **🤔 함수 시그니처의 수명 애노테이션**

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let msg1 = String::from("abcd");
    let msg2 = "xyz";
    println!("longest: {}", longest(msg1.as_str(), msg2));
}
// Result
// longest: abcd
```
- 제네릭 타입 매개변수와 마찬가지로 함수 이름과 매개변수 사이의 꺾쇠괄호에 제네릭 수명 매개변수를 선언해야 함
- 모든 참조 매개변수와 리턴값이 같은 수명을 가져야 하기 때문에 수명 `'a` 를 모든 참조에 지정함
- 수명 매개변수를 함수 시그니처에 명시한다고 해서 함수에 전달되거나 함수가 리턴하는 값의 수명을 변경하는 것이 아님
  - 대여 검사기가 이 제약에 일치하지 않는 값을 사용하지 못하도록 할 뿐

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let msg1 = String::from("abcd");
    {
        let msg2 = String::from("xyz");
        let result = longest(msg1.as_str(), msg2.as_str());
        println!("longest: {result}");
    }
}
// Result
// longest: abcd
```
- 리턴값에 대한 참조인 변수 result가 안쪽 범위에서 유효하기에 코드는 컴파일됨

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let msg1 = String::from("abcd");
    let result;
    {
        let msg2 = String::from("xyz");
        result = longest(msg1.as_str(), msg2.as_str());
    }
    println!("longest: {result}");
}
// Result
// error[E0597]: `msg2` does not live long enough
```
- 변수 result가 유효하려면 변수 msg2가 바깥쪽 범위가 끝날 때까지 유효해야 함
- 변수 msg2가 유효하지 않기에 유효하지 않은 참조를 리턴할 가능성이 있어서 컴파일되지 않음

<br>

#### **🤔 수명의 관점에서 생각하기**