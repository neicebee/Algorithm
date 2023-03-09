# 🦀 Rust Day 18

## **🏳️ Error Handling**
- 러스트는 에러의 발생 가능성을 인지하고 개발자가 코드를 컴파일하기 전에 에러를 처리하도록 유도함
- 대부분 언어는 에러를 구분하지 않고 `예외(exception)` 같은 메커니즘을 이용해 똑같이 처리함
- 러스트에 예외라는 개념은 존재하지 않으며, 에러를 두 가지로 구분하고 있음
- `회복 가능한 에러(recoverable error)`
  - 존재하지 않는 파일 등 회복 가능한 에러는 사용자에게 문제를 보고하고 작업을 다시 시도하도록 요청할 수 있음
  - `Result<T, E>` 타입으로 표현
- `회복이 불가능한 에러(unrecoverable error)`
  - 회복 불가능한 에러는 배열의 범위를 벗어나는 메모리에 대한 접근처럼 항상 버그의 가능성을 내포하고 있음
  - 해당 에러가 발생한 프로그램은 `panic!` 매크로를 바인딩하며 실행을 종료함

<br>

### **1️⃣ `panic!` 매크로를 이용한 회복 불가능한 에러 처리**
- 패닉은 대부분 어떤 종류의 버그가 발견되었는데 개발자가 이 에러를 처리할 방법이 마땅치 않을 때 활용
- `panic!` 매크로를 실행하면 프로그램은 실패 메시지를 출력하고, 스택을 모두 정리한 다음 종료함
- **패닉이 발생했을 때 스택을 풀어주거나(unwind) 취소하기**
  - 기본적으로 패닉이 발생하면 프로그램은 스택을 풀기 시작
    - 역순으로 순회하며 각 함수에 전달되었던 데이터 정리
    - 이를 위해 실행되는 작업의 양은 상당함
  - 스택을 즉시 취소해서 스택을 정리하지 않고 애플리케이션을 종료하는 방법
    - 프로그램이 사용하던 메모리는 OS가 정리해야 함
    - 프로그램의 바이너리 파일의 크기를 최대한 작게 해야 한다면 `Cargo.toml` 파일의 `[profile]` 섹션에 `panic = 'abort'` 를 추가하면 스택을 풀어주지 않고 취소할 수 있음
    - 릴리즈 모드에서 패닉을 취소할 시
        ```bash
        [profile.release]
        panic = 'abort'
        ```

```rust
fn main() {
    panic!("Error!");
}
```
- `panic!` 매크로를 호출하면 마지막 두 줄에 에러 메시지가 표시됨
- 첫 번째 줄에 패닉 메시지와 패닉이 발생한 소스 코드의 위치가 출력됨
- `src/main.rs:4:5` => `src/main.rs` 파일의 네 번째 줄 네 번째 문자

#### **🤔 `panic!` 역추적 사용하기**

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```
- 벡터의 크기를 벗어나는 인덱스를 이용해 값을 읽을 때, `panic!` 매크로 호출
- C 같은 다른 언어는 이런 경우 원치 않던 값이라도 개발자가 지정한 위치의 값을 리턴함
  - `버퍼 오버리드(buffer overread)`
    - 인덱스로 지정한 위치의 메모리가 벡터가 관리하는 메모리가 아니더라도 벡터에 저장된 값에 해당하는 위치의 메모리에 저장된 값을 리턴함
    - 공격자가 해당 방법으로 인덱스를 조작해서 읽어서는 안 되는 값을 읽게 되는 보안상의 취약점이 되기 쉬움
- `panic!` 매크로가 바인딩된 실행 결과에 있는 노트에는 `RUST_BACKTRACE` 환경 변수를 이용해 정확히 어떻게 에러가 발생했는지를 역추적할 수 있다는 것을 알려줌
  - 역추적은 그 지점까지 호출된 모든 함수의 목록
  - 제일 위의 함수부터 우리가 작성한 파일까지 함수의 호출 과정을 추적하는 것
- 역추적 정보를 얻으려면 `디버그 심볼(debug symbols)` 이 활성화되어 있어야 함
  - `cargo build` 나 `cargo run` 명령을 `--release` 플래그 없이 실행하면 기본적으로 활성화됨

<br>

### **2️⃣ `Result` 타입으로 에러 처리하기**
- 대부분 에러는 프로그램을 완전히 종료해야 할 정도로 치명적이지는 않음

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
- `Result` 열거자 정의 코드
- `T` 와 `E` 는 제네릭 타입 매개변수
- `T` 는 작업이 성공한 경우 `Ok` 열것값 안에 포함될 값의 타입
- `E` 는 작업이 실패한 경우 `Err` 열것값 안에 포함될 값의 타입

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("File Open Error: {:?}", error);
        }
    };
}
```
- match 표현식으로 Result 타입의 리턴값을 제어하는 코드
- `Option` 열거자와 마찬가지로 `Result` 열거자도 프렐류드에 의해 자동으로 추가됨

#### **🤔 `match` 표현식으로 여러 종류의 에러 처리하기**

```rust
use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("File create error!: {:?}", e),
            },
            others => panic!("File open error!: {:?}", others),
        },
    };
}
```
- 중첩된 `match` 표현식을 이용해 에러의 종류에 따라 다르게 처리하기
  - `File::open` 메서드가 리턴하는 `Err` 열것값에 저장된 값의 타입은 `io::Error` 타입
  - `io::Error` 구조체는 `io::ErrorKind` 타입을 리턴하는 `kind` 메서드 제공
  - `ErrorKind::NotFound` : 존재하지 않는 파일을 열고자 할 때 발생하는 열것값
  - `error.kind()` 가 리턴한 값이 `ErrorKind::NotFound` 열것값일 때 파일 생성
  - 파일 생성이 실패할 경우를 대비해 `match` 표현식에 가지를 추가함
  - `ErrorKind::NotFound` 에러를 제외한 다른 에러일 경우 프로그램이 패닉에 빠짐

#### **🤔 에러 발생 시 패닉을 발생하는 더 빠른 방법: `unwrap` 과 `expect`**
- `Result<T, E>` 타입은 다양한 작업을 처리하기 위한 `unwrap` 메서드와 `expect` 메서드 제공
  - `unwrap` 메서드
    - 중첩된 `match` 표현식과 정확히 같은 동작을 하는 `단축(shortcut)` 메서드
    - `Result` 타입이 `Ok` 열것값일 때 `Ok` 열것값에 저장된 값 리턴
    - `Result` 타입이 `Err` 열것값일 때는 `panic!` 매크로 호출
  - `expect` 메서드
    - `unwrap` 메서드와 유사하지만 `panic!` 매크로에 에러 메시지 전달 가능
    - 개발자의 의도를 더 명확하게 표현하는 동시에 패닉이 발생한 원인을 더 쉽게 추적 가능

#### **🤔 에러 전파하기**

```rust
use std::io::{self, Read};
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("{:?}", read_username_from_file());
}
```
- `match` 표현식을 이용해 호출자에게 에러를 리턴하는 `read_username_from_file` 함수
- 해당 함수의 리턴 타입은 `Result<String, io::Error>`
  - `Result<T, E>` 타입
  - 제네릭 매개변수 `T` : `String`
  - 제네릭 매개변수 `E` : `io:Error`
- 러스트는 에러 전파하기 작업을 더 쉽게 처리할 수 있도록 `?` 연산자 제공

**(1)** `?` 연산자를 이용해 에러를 더 쉽게 전파하기

```rust
use std::io::{self, Read};
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    println!("{:?}", read_username_from_file());
}
```
- `Result` 값 뒤에 붙인 `?` 연산자는 `match` 표현식을 이용한 처리와 거의 같은 방식으로 동작함
- `Result` 값이 `Ok`
  - `Ok` 열것값에 저장된 값 리턴 후 계속 실행
- `Result` 값이 `Err`
  - `Err` 열것값이 `return` keyword를 사용한 것처럼 전체 함수의 리턴값이 됨
- `match` 표현식과 `?` 연산자 동작의 차이점
  - `?` 연산자의 에러값은 `from` 함수를 이용해 전달됨
    - `from` 함수
      - 표준 라이브러리에 정의된 `From` 트레이트에 선언되어 있음
      - 에러를 어떤 한 타입에서 다른 타입으로 변환함
  - `?` 연산자가 `from` 함수를 호출하면 전달된 에러 타입은 현재 함수의 리턴 타입에 정의된 에러 타입으로 변환됨

```rust
use std::io::{self, Read};
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    println!("{:?}", read_username_from_file());
}
```
- `?` 연산자 뒤에 메서드를 연결하여 호출하면 코드를 더 간결하게 작성이 가능함

```rust
use std::{io, fs};

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    println!("{:?}", read_username_from_file());
}
```
- 파일을 문자열로 읽는 작업은 매우 빈번한 작업
- 때문에 러스트는 `fs::read_to_string` 함수를 제공함
- 간결하지만 앞서 작성한 코드와 같은 동작을 수행함

<br>

**(2)** `?` 연산자는 `Result` 타입을 리턴하는 함수에서만 사용할 수 있다
- `?` 연산자는 `match` 표현식과 같은 동작을 실행하도록 정의되어 있기 때문
  - 함수의 리턴 타입은 반드시 `Result` 타입이어야 함
- `Result` 타입을 리턴하지 않는 함수 안에서 `Result` 타입을 리턴하는 다른 함수를 호출할 때는 호출자에게 에러를 전파할 가능성이 있는 `?` 연산자를 사용하기보다 `match` 표현식이나 `Result` 타입의 메서드 중 하나를 사용해서 처리해야 함

```rust
use std::{fs::File, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("hello.txt")?;

    Ok(())
}
```
- `main` 함수는 리턴할 수 있는 값의 타입에 제한이 있음
  - 그 중 하나는 `()` 이며, `Result<T, E>` 타입을 리턴할 수도 있음
- `Box<dyn Error>` 타입은 `트레이트 객체(trait object)` 타입
  - '모든 종류의 에러' 의미

<br>

### **3️⃣ 패닉에 빠질 것인가? 말 것인가?**
- `Result` 값을 리턴하면 직접 결정을 내리는 것이 아니라 호출자에게 선택지를 줄 수 있음
  - 적절한 상황인 경우 에러로부터 회복을 시도
  - 해당 에러로부터 회복할 수 없다는 결론을 내릴 경우 패닉 결정
- `Result` 타입을 리턴하는 것보다 그냥 패닉을 발생시키는 것이 더 적절할 때도 있음

#### **🤔 예제, 프로토타입 코드, 그리고 테스트**
- `unwrap` 과 `expect` 메서드는 에러를 어떻게 처리할 것인지 결정하기에 앞서 프로토타이핑을 해보는 상황에 매우 유용함
- 테스트 코드를 실행하는 중에 메서드 호출이 실패하면 전체 테스트를 실패처리 해야 할 것임
  - `panic!` 매크로는 테스트가 실패했음을 표시할 때도 사용하므로 `unwrap` 이나 `expect` 메서드를 이용해도 마찬가지임

#### **🤔 컴파일러보다 개발자가 더 많은 정보를 가진 경우**
- `Result` 타입의 결과가 `Ok` 값임이 확실하더라도, 컴파일러가 이해할 수 없는 로직이라면 `unwrap` 메서드를 함께 호출해주는 것이 좋음
- 호출하는 작업이 특정 상황에서는 실패할 리가 없더라도 통상적으로 그 작업은 얼마든지 실패할 가능성이 있기 때문

#### **🤔 에러 처리를 위한 조언**
- 코드가 결국은 잘못된 상태가 될 상황이라면 패닉을 발생시키는 것도 나쁜 선택이 아님
  - **잘못된 상태**
    - 어떤 가설이나 보장, 계약 혹은 불변이어야 할 것들이 깨진 상황
    - 유효하지 않은 값, 모순된 값, 혹은 실수로 놓친 값이 코드로 전달되는 상황
    - 잘못된 상태는 원래 기대했던 동작이 어쩌다 실패하는 상황을 말하는 것이 아님
    - 어느 지점 이후의 코드는 프로그램이 절대 잘못된 상태에 놓이지 않아야만 정상적으로 동작함
    - 이 정보를 사용 중인 타입으로 표현할 방법이 없음
- 어떤 이유로 작업이 실패할 수 있다면 `panic!` 매크로를 호출하는 것보다는 `Result` 타입을 리턴하는 편이 나음
- 코드가 어떤 값에 대한 작업을 수행할 때는 그 값이 유효한지를 반드시 먼저 검사한 후 그렇지 않으면 패닉을 발생시켜야 함
  - 가장 큰 이유는 안전성
- 모든 함수에서 가능한 에러를 모두 검사하는 것도 힘든 작업임
  - 러스트의 타입 시스템(그리고 컴파일러의 타입 검사)을 이용해 여러 가지 검사 수행 가능
    - 함수 매개변수의 타입 같은 경우는 컴파일러가 해당 타입의 유효한 값이 전달되는 것이 보장된다는 것을 알고 있으므로 추가 검사 없이 로직을 구현해 나가면 됨

#### **🤔 유효성 검사를 위한 커스텀 타입**
- 새로운 타입을 생성하고 이 타입의 인스턴스를 생성하는 함수에 유효성 검사 코드를 작성하는 것이 매우 효율적인 방법

<br>

## **Summary**
- `panic!` 매크로
  - 프로그램이 제대로 처리할 수 없는 비정상적인 상태에 놓였다는 것을 알려주고 유효하지 않거나 잘못된 값을 계속 사용하지 못하도록 프로세스를 종료함
- `Result` 열거자
  - 러스트의 타입 시스템을 이용해 작업이 실패할 수도 있음을 명시하고, 실패한 경우 프로그램을 회복할 기회를 제공함
  - `Result` 타입을 리턴해서 호출 코드가 작업이 성공한 경우와 실패한 경우를 모두 처리할 수 있도록 유도 가능