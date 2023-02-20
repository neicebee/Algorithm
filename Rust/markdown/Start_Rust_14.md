# 🦀 Rust Day 14

## **🏳️ Common Collections**
- 내장된 배열과 튜플과는 달리 컬렉션이 가리키는 데이터는 힙 메모리에 저장됨
  - 데이터의 크기를 컴파일 시점에 알 필요가 없으며, 프로그램의 실행 중에 얼마든지 그 크기를 증가시키거나 감소시킬 수 있음
- `벡터(vector)` : 연속된 일련의 값을 저장함
- `문자열(string)` : `문자(character)` 의 컬렉션
- `해시 맵(hash map)` : 특정 키에 값을 연결할 때 사용. 해시 맵은 더 범용으로 사용되는 `맵(map)` 을 구현한 구현체임

<br>

### **1️⃣ 벡터에 일련의 값 저장하기**
- `Vec<T>` 타입
  - 벡터를 이용하면 하나 이상의 값을 하나의 데이터 구조에 담을 수 있으며 모든 값은 메모리상에 연속으로 저장됨
  - 어떤 타입이든 저장 가능
  - 특정한 타입 저장 시 해당 타입에 대한 애노테이션을 명시해주어야 함
  - 같은 타입의 값만을 저장할 수 있음

#### **🤔 새로운 벡터 생성하기**

```rust
fn main() {
    let v: Vec<i32> = Vec::new();
}
```
- `Vec::new` 함수로 새로운 빈 벡터 생성
- 타입에 대한 애노테이션 명시
- 벡터는 `제네릭(generic)` 을 이용해 구현됨
- 러스트는 벡터에 일단 값을 추가하면 그 타입을 유추할 수 있음
  - 빈 벡터를 생성하지 않는 이상 타입 애노테이션을 사용할 일이 별로 없음

```rust
fn main() {
    let v = vec![1, 2, 3];
}
```
- 지정한 값을 저장하는 새로운 백터를 생성하는 `vec!` 매크로
- 초깃값 지정 시 `i32` 타입의 값을 지정했으므로 타입 애노테이션을 추가할 필요가 없음

#### **🤔 벡터 수정**

```rust
fn main() {
    let mut v = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);
}
```
- `push` 메서드를 이용해 벡터에 값 추가

#### **🤔 벡터 해제**
- 다른 구조체와 마찬가지로 벡터 역시 범위를 벗어날 때 `drop` 메서드가 호출됨
- 벡터가 메모리에서 해제되면 벡터에 저장된 모든 값도 함께 해제됨
  - 모든 정수값도 해제됨
- 벡터에 참조형 값을 저장하는 경우에는 일이 복잡해질 수 있음

#### **🤔 벡터로부터 값 읽기**

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("세 번째 원소 : {third}");
}
// Result
// 세 번째 원소 : 3
```
- `인덱스 문법(indexing syntax)` 이용
- `&` 와 `[]` 를 이용해 저장된 값에 대한 참조를 리턴


```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    match v.get(2) {
        Some(third) => println!("세 번째 원소 : {third}"),
        None => println!("세 번째 원소가 없음."),
    }
}
// Result
// 세 번째 원소 : 3
```
- `get` 메서드 이용
- `Option<&T>` 타입의 값 리턴

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let no_value = &v[100];
    let no_value = v.get(100);
}
```
- 벡터의 존재하지 않는 인덱스의 값에 접근할 때 `[]` 방식은 `패닉(panic)` 발생
  - 지정된 참조가 존재하지 않는 값을 가리키기 때문
- `get` 메서드로 존재하지 않는 인덱스의 값에 접근하면 `None` 값이 리턴됨

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);
}
```
- **_필자의 `rustc` 버전은 `1.62.0` 인데 에러가 발생하지 않음_**
  - 일단 개념은 정리해두려고 함
- 프로그램이 유효한 참조값을 얻게 되면 벡터에 저장된 값에 대한 참조가 계속해서 유효할 수 있도록 대여값 검사가 실행되어 소유권과 대여 규칙을 적용함
- 같은 범위 내에서 가변 참조와 불변 참조를 동시에 가질 수 없음
- 현재 벡터의 크기가 충분히 크지 않다면 벡터의 마지막에 새로운 값을 추가하기 위해 새로운 메모리를 할당하고 이미 저장된 값들을 새로운 메모리 공간으로 옮겨야 할 수도 있음

#### **🤔 벡터에 저장된 값을 순회하기**

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];

    for i in &v {
        println!("{i}");
    }
}
// Result
// 1
// 2
// 3
// 4
// 5
// 6
```
- `for` 문을 이용해 벡터에 저장된 값에 대한 불변 참조를 얻어와 출력함

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }
}
// Result
// 51
// 52
// 53
// 54
// 55
// 56
```
- 가변 벡터에 저장된 값에 대한 가변 참조를 얻어와 값을 변경함
- 가변 참조가 가리키는 값을 변경하려면 `+=` 연산자 사용 전에 `역참조 연산자(*)` 를 이용해 변수 `i` 에 저장된 값을 가져와야 함

#### **🤔 열거자를 이용해 여러 타입 저장하기**
- 열거자의 열것값은 같은 열거자 타입으로 평가되므로 벡터에 다른 타입의 값을 저장하려면 열거자를 정의해 사용하면 됨

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("가나다라")),
    ];
}
```
- 열거자를 이용해 하나의 벡터에 다른 타입의 값 저장
- 벡터에 어떤 값을 저장할 수 있는지 명시할 수 있음
  - 러스트는 컴파일 시점에 벡터에 어떤 타입의 값이 저장될지 알아야 하므로 당연히 각각의 값을 저장하기 위해 어느 정도의 힙 메모리가 필요한지도 정확히 판단함
- 프로그램이 벡터에 저장해야 할 타입들을 프로그램을 작성하는 시점에 완벽히 알 수 없다면 열거자를 이용하는 방법은 별다른 도움이 되지 않음

<br>

### **2️⃣ `String` 타입에 `UTF-8` 형식의 텍스트 저장하기**
- 러스트의 문자열은 일반적으로 개발자들이 알고 있는 것보다 더 복잡한 데이터 구조이며, `UTF-8` 형식으로 저장됨
- 러스트의 문자열은 `바이트(bytes)` 의 컬렉션으로 구현되어 있을 뿐만 아니라, 이 바이트를 문자열로 처리할 때 유용한 여러 가지 메서드를 제공함

#### **🤔 문자열이란 무엇일까?**
- 러스트는 언어의 명세 안에서 오직 한 가지의 문자열 타입, 즉 문자열 슬라이스인 `str` 타입만을 지원함
  - 주로 값을 대여한 `&str` 의 형태로 보게 될 것임
  - 어딘가에 `UTF-8` 형식으로 인코딩되어 저장된 문자열에 대한 참조
  - 문자열 리터럴은 프로그램을 컴파일한 바이너리 결과 파일에 포함되므로 문자열 슬라이스임
- `String` 타입은 언어의 명세에 정의된 것이 아니라 러스트의 표준 라이브러리가 제공하는 타입
  - 길이 조정이나 내용 변경, 소유 등이 가능하며 `UTF-8` 형식으로 인코딩된 문자열 타입
- 러스트의 표준 라이브러리는 `OsString`, `OsStr`, `CString`, `CStr` 과 같은 다른 종류의 문자열 타입도 제공
  - 이 문자열 타입들은 다른 인코딩 형식의 텍스트를 저장하거나 메모리상에 다른 형태로 표현되기도 함

#### **🤔 새 문자열 생성하기**
- `String` 타입은 `Vec<T>` 타입이 지원하는 대부분 작업을 지원함

```rust
fn main() {
    let mut s = String::new();
}
```
- 새로운 빈 문자열을 생성해 변수 s에 할당

```rust
fn main() {
    let data = "First String";
    let s = data.to_string();
    // let s = "First String".to_string();
}
```
- 문자열 생성 시 초깃값 지정
- `Display` 크레이트를 구현한 모든 타입에서 사용할 수 있는 `to_string` 함수 활용

```rust
fn main() {
    let mut s = String::from("First String");
}
```
- 문자열 리터럴로 `String` 타입 생성 시 `from` 함수 이용
- `String::from` 과 `to_string` 은 같은 작업을 수행하므로 자신의 스타일에 따라 선택해 사용하면 됨

#### **🤔 문자열 수정하기**
**(1)** `push_str` 과 `push` 메서드를 이용해 문자열 덧붙이기

```rust
fn main() {
    let mut s = String::from("foot");
    s.push_str("ball");

    println!("{s}");
}
// Result
// football
```
- `push_str` 메서드는 문자열 슬라이스를 `String` 에 덧붙임
- `push_str` 메서드가 문자열 슬라이스를 이용하는 이유는 매개변수의 소유권을 가질 필요가 없기 때문

```rust
fn main() {
    let mut s1 = String::from("foot");
    let s2 = "ball";
    s1.push_str(s2);

    println!("s1: {s1}\ns2: {s2}");
}
// Result
// s1: football
// s2: ball
```
- `push_str` 메서드가 변수 s2에 대한 소유권을 갖게 된다면 출력문에서 변수 s2는 유효하지 않게 됨

```rust
fn main() {
    let mut s = String::from("coffe");
    s.push('e');

    println!("{s}");
}
// Result
// coffee
```
- `push` 메서드는 하나의 `문자(character)` 를 매개변수로 전달받아 `String` 에 추가

<br>

**(2)** `+` 연산자나 `format!` 매크로를 이용한 문자열 연결

```rust
fn main() {
    let s1 = String::from("Football");
    let s2 = String::from(" Manager");
    let s3 = s1 + &s2;
    println!("{s3}");
}
// Result
// Football Manager
```
- 변수 s1은 유효하지 않음
  - `+` 연산자를 사용하면 내부적으로 `add` 메서드가 호출됨
  - `fn add(self, s: &str) -> String {`
    - 표준 라이브러리는 `add` 메서드를 제네릭을 이용해 선언함
    - 해당 시그니처는 `String` 을 이용해 `add` 메서드를 호출할 때 제네릭 타입이 실제 사용한 타입으로 치환된 이후의 시그니처임
    - 두 번째 문자열의 **_참조_** 를 첫 번째 문자열에 추가하고 `String` 타입을 리턴함
    - `add` 메서드의 매개변수 s가 참조 형식이므로 `String` 에는 오직 `&str` 타입의 값만 추가 가능하며 두 개의 `String` 값을 결합할 수 없음
    - 위 코드에서 변수 s2의 타입은 `&String` 인데 매개변수로 전달할 수 있었던 이유는 컴파일러가 `&String` 인수를 `&str` 타입으로 변환하기 때문
    - `add` 메서드 호출 시 러스트는 `강제 역참조(deref coercion)` 를 이용하여 `&s2` 를 `&s2[..]` 로 변환함
  - 즉, s1의 소유권을 확보한 후 s2의 값을 복사해서 덧붙인 후 그 결괏값에 대한 소유권을 리턴하는 것임

```rust
fn main() {
    let s1 = String::from("one");
    let s2 = String::from("two");
    let s3 = String::from("three");

    // let add_s = s1 + "-" + &s2 + "-" + &s3;
    let format_s = format!("{}-{}-{}", s1, s2, s3);
    println!("{format_s}");
}
// Result
// one-two-three
```
- 문자열을 여러 개 결합할 때, `+` 연산자는 그다지 효율적이지 못할 수 있음
- `format!` 매크로는 `println!` 매크로와 완전히 같은 방식으로 동작하지만, 화면에 결과를 출력하는 대신 결합된 `String` 값을 리턴함
- `format!` 매크로는 가독성이 좋을 뿐 아니라 매개변수의 소유권도 없음

<br>

#### **🤔 문자열의 인덱스**
- 다른 프로그래밍 언어에서는 문자열에 저장된 개별 문자를 인덱스를 이용해 접근할 수 있음
- 러스트에서는 인덱스를 이용해 `String` 값의 일부에 접근하려 하면 에러가 발생함
- 러스트의 문자열은 인덱스를 지원하지 않음

<br>

**(1)** 문자열의 내부
- `String` 은 `Vec<u8>` 타입을 한 번 감싼 타입임

```rust
let len = String::from("Hello").len();
// Result
// 5
```
- 변수 len의 값은 5
- 벡터에 저장된 문자열 'Hello' 의 길이가 5bytes
- `UTF-8` 로 인코딩된 문자들은 한 글자당 1byte

```rust
let len = String::from("안녕하세요").len();
```
- 러스트에서 이 문자열의 길이는 15
- 해당 문자열을 `UTF-8` 로 인코딩하면 15bytes 사용
  - 문자열 안 유니코드의 스칼라값은 3bytes 공간을 사용하기 때문
  - 문자열의 바이트에 인덱스로 접근하면 올바른 유니코드 스칼라값을 가져오기가 불가능할 수 있음

```rust
let s = String::from("안녕하세요");
let ans = &s[0];
```
- 해당 문자열을 `UTF-8` 형식으로 인코딩하면 첫 번째 바이트는 236이고 두 번째 바이트는 149
- 러스트는 인덱스 0을 통해 알 수 있는 값은 첫 번째 단어가 아닌 바이트값 236뿐임
- 때문에 러스트는 이러한 코드를 컴파일하지 않음

<br>

**(2)** 바이트와 스칼라값, 그리고 그래핌 클러스터
- 러스트 관점에서 볼 때 문자열은 크게 `바이트(bytes)`, `스칼라값(scalar values)`, `그래핌 클러스터(grapheme clusters, 우리가 문자라고 부르는 것에 가장 가까운 것)` 등 세 가지로 구분함
  - **_그래핌이란, 어떤 언어의 철자 체계에서 가장 작은 단위를 의미함. 알파벳의 경우 각 자모가 이에 해당함._**
- 한글 '안녕하세요' 의 `u8` 값들
  - `[236, 149, 136, 235, 133, 149, 237, 149, 152, 236, 132, 184, 236, 154, 148]`
  - 총 15개의 바이트값이 컴퓨터가 최종적으로 이 데이터를 저장하는 형태
- 15개의 바이트값을 러스트의 `char` 타입인 유니코드 스칼라값으로 표현
  - `['안', '녕', '하', '세', '요']`
  - 총 5개의 `char` 값
- 15개의 바이트값을 그래핌 클러스터로 표현
  - `["안", "녕", "하", "세", "요"]`
- 러스트가 `String` 타입에서 인덱스 사용을 지원하지 않는 마지막 이유
  - 인덱스 처리에는 항상 일정한 시간($O(1)$)이 소요되어야 하지만 `String` 타입에 대해서만큼은 일정한 성능을 보장할 수 없음
  - 러스트는 유효한 문자를 파악하기 위해 콘텐츠를 처음부터 스캔해야 하기 때문

<br>

#### **🤔 문자열 슬라이스 하기**
- 문자열에 대한 인덱싱 작업은 상황에 따라 결과 타입이 하나의 바이트값, 하나의 문자, 하나의 그래핌 클래스터 혹인 하나의 문자열 슬라이스 중 하나가 될 수 있어 명확하지 않음
- 인덱스를 이용해 문자열 슬라이스를 생성하려면 `[]` 를 이용해 문자열 슬라이스에 저장할 **_특정 바이트_** 의 범위를 명확히 지정해야 함

```rust
fn main() {
    let s = String::from("안녕하세요");
    println!("{}", &s[0..3]);
}
// Result
// 안
```
- 변수 s는 문자열의 처음 3bytes 값을 저장하는 `&str` 타입
- `&s[0..1]` 로 코드를 작성하게 되면 벡터에 유효하지 않은 인덱스로 접근할 때와 동일하게 패닉이 발생하게 됨

#### **🤔 문자열을 순회하는 메서드**

```rust
fn main() {
    let s = String::from("안녕하세요");
    for c in s.chars() {
        println!("{c}");
    }
}
// Result
// 안
// 녕
// 하
// 세
// 요
```
- 개별 유니코드 스칼라값을 조작해야 한다면 `chars` 메서드 사용
- 유효한 유니코드 스칼라값은 1byte보다 큰 값으로 구성됨
- `chars` 메서드는 문자열을 `char` 타입으로 분리함

```rust
fn main() {
    let s = String::from("안녕하세요");
    for b in s.bytes() {
        println!("{b}");
    }
}
```
- `bytes` 메서드는 문자열의 각 바이트를 리턴함

<br>

### **3️⃣ 키와 값을 저장하는 해시 맵**
- `HashMap<K, V>` 타입은 `K` 타입의 키에 `V` 타입의 값을 매핑하여 저장
- 유사한 형태의 데이터 구조
  - 해시, 맵, 객체, 해시 테이블(hash table), 딕셔너리(dictionary), 연관 배열(associative array) 등
- 해시 맵은 벡터처럼 인덱스를 이용하는 것이 아니라 어떤 타입이든 관계없이 키를 이용하여 데이터를 조회하고자 할 때 유용

#### **🤔 새로운 해시 맵 생성하기**

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(
        String::from("Blue"),
        20
    );
    scores.insert(
        String::from("Yellow"),
        50
    );

    println!("{:#?}", scores);
}
// Result
// {
//     "Yellow": 50,
//     "Blue": 20,
// }
```
- `new` 함수로 빈 해시 맵 생성
- `insert` 함수로 새로운 키와 값을 추가
- 해시 맵은 사용빈도가 낮아 프렐류드를 통해 자동으로 현재 범위로 가져오는 기능이 포함되어 있지 않으며, 표준 라이브러리의 지원도 빈약함
- 해시 맵은 벡터와 마찬가지로 데이터를 힙 메모리에 저장하며, 모든 키와 모든 값의 타입이 같아야 함

```rust
use std::collections::HashMap;

fn main() {
    let teams = vec![
        String::from("Blue"),
        String::from("Yellow")
    ];
    let score = vec![20, 50];

    let scores: HashMap<_, _> = teams.iter()
        .zip(score.iter()).collect();

    println!("{:#?}", scores);
}
// Result
// {
//     "Blue": 20,
//     "Yellow": 50,
// }
```
- 키와 값을 가지고 있는 튜플의 벡터에 대해 `collect` 메서드를 호출
- `collect` 메서드
  - 여러 가지 종류의 컬렉션으로부터 데이터를 수집함
  - 여러 가지 데이터 구조를 생성할 수 있으므로 그 중 어떤 타입을 생성할 것인지 명시하는 타입 애노테이션 필요
- 타입 애노테이션 중 매개변수에 대해서 `밑줄(_)` 을 사용하여 벡터의 데이터 타입을 이용해 키와 값의 타입을 알아서 유추할 수 있도록 함

```rust
fn main() {
    let teams = vec![
        String::from("Blue"),
        String::from("Yellow")
    ];
    let score = vec![20, 50];

    let scores = teams.iter().zip(score.iter());

    for i in scores {
        println!("{:#?}", i);
    }
}
// Result
// (
//     "Blue",
//     20,
// )
// (
//     "Yellow",
//     50,
// )
```
- `zip` 메서드로는 튜플의 벡터를 생성할 수 있음

#### **🤔 해시 맵과 소유권**
- `i32` 처럼 `Copy` 트레이트를 구현하는 타입은 값들이 해시 맵으로 복사됨
- `String` 처럼 값을 소유하는 타입은 값이 해시 맵으로 이동하며, 해시 맵이 그 값들의 소유권을 갖게 됨
- 해시 맵에 값의 참조를 추가하면 그 값은 해시 맵으로 이동하지 않음
  - 참조가 가리키는 값은 해시 맵이 유효한 범위에 있는 동안 함께 유효해야 함

#### **🤔 해시 맵의 값에 접근하기**

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Yellow"), 50);

    let score = scores.get(
        &String::from("Blue")
    );

    println!("{:#?}", score);
}
// Result
// Some(
//     20,
// )
```
- `get` 메서드에 키를 전달하면 값에 접근 가능
- `get` 메서드는 `Option<&V>` 타입을 리턴함
  - 즉, 해시 맵의 키에 대한 값이 존재하지 않으면 `None` 을 리턴

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key} : {value}");
    }
}
// Result
// Yellow : 50
// Blue : 20
```
- `for` 루프를 이용한 해시 맵 키와 값의 쌍 순회 출력
- 해시 맵의 키와 값의 쌍을 **_임의의 순서_** 로 출력함

#### **🤔 해시 맵 수정하기**
- 해시 맵의 각 키에는 오직 하나의 값만 할당할 수 있음

<br>

**(1)** 값 덮어쓰기
- 해시 맵에 키와 값을 추가한 후 같은 키에 다른 값을 추가하면 키에 할당되었던 값이 교체됨

<br>

**(2)** 키에 값이 할당되어 있지 않을 때만 추가하기
- 해시 맵은 값의 할당 여부를 확인할 키를 매개변수로 사용하는 `entry` 라는 특별한 API를 제공함
- `entry` 메서드의 리턴값은 값이 존재하는지 알려주는 `Entry` 열거자임

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 20);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:#?}", scores);
}
// Result
// {
//     "Blue": 20,
//     "Yellow": 50,
// }
```
- `Entry` 열거자의 `or_insert` 메서드는 키가 존재하면 그 키에 연결된 값에 대한 가변 참조를 리턴
  - 존재하지 않을 시 매개변수로 전달한 키에 새로운 값을 추가한 후 이 새 값에 대한 가변 참조 리턴

<br>

**(3)** 기존 값에 따라 값 수정하기

```rust
use std::collections::HashMap;

fn main() {
    let s = "hello world pretty wonderful world";
    let mut map = HashMap::new();

    for word in s.split(' ') {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}
// Result
// {
//     "wonderful": 1,
//     "pretty": 1,
//     "hello": 1,
//     "world": 2,
// }
```
- 텍스트 안에서 각 단어가 몇 번 사용되었는지 세는 코드
- `or_insert` 메서드는 `키에 할당된 값에 대한 가변 참조(&mut V)` 를 리턴
  - 해당 가변 참조를 count 변수에 저장했으므로 이 변수에 새 값을 할당하려면 `애스터리스크(*)` 를 이용해 count 변수를 역참조해야 함
  - 가변 참조는 `for` 루프가 끝나면 범위 밖으로 나가게 되므로 값 대여 규칙을 위반하지 않으며 안전하게 값을 변경할 수 있음

<br>

#### **🤔 해시 함수**
- 해시 맵은 암호학적으로 강력한 해시 함수를 이용하여 `서비스 거부(DoS, Denial of Service)` 공격을 방지할 수 있음
- 가장 빠른 해싱 알고리즘을 사용하지는 않지만, 어느 정도의 성능을 희생하면서 보안을 향상시키는 것은 필요함

<br>

## **Summary**
- 벡터, 문자열, 해시 맵은 대부분 프로그램이 데이터를 저장하고 읽고 수정하는 데 필요한 방대한 기능을 제공함