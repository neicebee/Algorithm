# 🦀 Rust Day 21

## **🏳️ An I/O Project: Building a Command Line Program**
- 러스트의 속도와 안전성, 단일 바이너리 출력 그리고 교차 플랫폼 지원 등의 특징은 명령줄 도구를 만드는 데 좋음
- `grep(globally search a regular experssion and print)`

<br>

### **1️⃣ 명령줄 인수 처리하기**
- 프로젝트 이름 : `minigrep`
  - 파일명과 검색할 문자열 등 두 개의 명령줄 인수를 처리
    - `cargo run [searchstring] [example-filename.txt]`

#### **🤔 인수의 값 읽어오기**
- `std::env::args` : 표준 라이브러리에서 제공하는 전달된 명령줄 인수를 읽는 함수
  - 명령줄 인수의 `반복자(iterator)` 제공

```rust
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    println!("{:?}", args);
}
```
- 원하는 함수가 하나 이상의 모듈에 중첩되었을 때는 함수 대신 부모 모듈을 범위로 가져오는 것이 편리함
- `std::env::args` 함수는 인수에 유효하지 않은 유니코드가 포함되어 있으면 패닉을 발생시킴
- `std::env::args_os` 함수는 유효하지 않은 유니코드를 허용하지만 String이 아닌 OsString 값의 반복자를 리턴함
  - OsString 값은 플랫폼마다 다르며 String보다 훨씬 복잡함

```bash
Input:
    cargo run
Output:
    ["target/debug/minigrep"]
Input:
    cargo run Hi, bash!
Output:
    ["target/debug/minigrep", "Hi,", "bash!"]
```
- 벡터는 바이너리의 이름을 선두로 공백을 기준으로 인자가 나열되어 있음

#### **🤔 인숫값을 변수에 저장하기**

```rust
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let q = &args[1];
    let f = &args[2];
    println!("검색어: {q}\n대상 파일: {f}");
}
```

```bash
Input: 
    cargo run test sample.txt
Output:
    검색어: test
    대상 파일: sample.txt
```

<br>

### **2️⃣ 파일 읽기**

```
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```
- 같은 텍스트가 여러 줄에 걸쳐 반복된 작은 크기의 텍스트 파일 `poem.txt`

```rust
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let q = &args[1];
    let f = &args[2];
    let c = std::fs::read_to_string(f).unwrap();
    println!("검색어: {q}\n대상 파일: {f}\n파일 내용:\n{c}");
}
```
- `std::fs` : 파일을 처리하는 모듈

<br>

### **3️⃣ 모듈화와 에러 처리 향상을 위한 리팩토링**
- 함수 하나가 n 가지의 작업을 모두 처리하면 비효율적임
  - 함수의 목적을 명확히 하기가 어려워지는 것은 물론, 테스트도 쉽지 않거니와 다른 동작에 영향을 주지 않으면서 코드를 변경하기도 어려워짐
  - 함수가 길어질수록 더 많은 변수를 선언해야 하고, 범위에 더 많은 변수를 선언할수록 각 변수의 목적을 명확히 하기 어려워짐
    - 설정 변수를 하나의 구조체에 모아서 그 목적을 명확히 하는 것이 좋음

#### **🤔 바이너리 프로젝트의 관심 분리**
- 바이너리 프로그램 main 함수의 크기가 증감에 따라 관심을 분리하기 위한 지침
  - _프로그램을 main.rs와 lib.rs 파일로 분리하고 프로그램의 로직을 lib.rs 파일로 옮긴다._
  - _명령줄 구문분석 로직이 충분히 작다면 main.rs 파일에 남겨둔다._
  - _명령줄 구문분석 로직이 복잡해지기 시작하면 main.rs 파일에서 추출해 lib.rs 파일로 옮긴다._
- main 함수에 남겨지는 역할
  - _인숫값을 이용해 명령줄 구문분석 로직을 호출_
  - _기타 다른 설정 적용_
  - _lib.rs 파일의 run 함수 호출_
  - _run 함수가 에러를 리턴할 경우 이에 대한 처리_
- **_main.rs 파일은 프로그램의 실행을 담당하며 lib.rs 파일은 처리할 모든 작업의 로직을 담당_**

<br>

**(1)** 인수 구문분석 분리하기
- main.rs 파일에서 새로운 함수 제작

**(2)** 설정값의 그룹화
- `복합 타입(complex type)` 을 사용하는 편이 더 적절한 상황에서 기본 자료형을 사용하는 것은 `기본 자료형 강박(primitive obsession)` 이라는 `안티 패턴(anti-pattern)` 임

**(3)** 구조체의 생성자

```rust
use std::env;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args);
    let c = std::fs::read_to_string(&config.filename).unwrap();
    println!("검색어: {}\n대상 파일: {}\n파일 내용:\n{c}", config.query, config.filename);
}
```

```bash
검색어: the
대상 파일: poem.txt
파일 내용:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

<br>

#### **🤔 에러 처리 개선하기**

```rust
use std::env;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(
        |err| {
            println!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
            process::exit(1);
        }
    );
    let c = std::fs::read_to_string(&config.filename).unwrap();
    println!("검색어: {}\n대상 파일: {}\n파일 내용:\n{c}", config.query, config.filename);
}
```
- `unwrap_or_else`
  - 표준 라이브러리가 `Result<T, E>` 타입에 정의한 메서드
  - Result 값이 Ok 값이면 unwrap 메서드와 유사하게 동작함
    - Ok 열것값에 저장된 값 리턴
  - Err 값이면 클로저를 이용해 unwrap_or_else 메서드에 전달한 익명 함수를 호출함
    - 정적 문자열을 익명 함수의 파이프 문자 사이에 선언한 인수에 전달

```bash
인수를 구문분석하는 동안 오류가 발생했습니다: 필요한 인수가 지정되지 않았습니다.
```

#### **🤔 main 함수에서 로직 분리하기**