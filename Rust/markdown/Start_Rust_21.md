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

fn run(config: Config) {
    let c = std::fs::read_to_string(config.filename)
        .unwrap();
    println!("파일 내용:\n{c}");
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(
        |err| {
            println!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
            process::exit(1);
        }
    );
    println!("검색어: {}\n파일 이름: {}", config.query, config.filename);
    run(config);
}
```
- run 함수로 로직 분리

```rust
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let c = std::fs::read_to_string(config.filename)?;
    println!("파일 내용:\n{c}");
    Ok(())
}
```
- Result 타입을 리턴하는 run 함수
- Ok 상황에서는 유닛 타입 `()` 리턴
- Err 상황에서는 트레이트 객체인 `Box<dyn Error>` 리턴
  - `Box<dyn Error>` : Error 트레이트를 구현하는 타입을 리턴하지만, 리턴될 값의 타입을 특정하지는 않음
    - 여러 에러 상황에서 각기 다른 타입을 리넡할 수 있는 유연성을 확보할 수 있음
- expect 메서드 대신에 `?` 연산자 사용
  - `?` : 에러가 발생할 때 현재 함수의 호출자에게 에러값을 리턴할 수 있음

```rust
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(
        |err| {
            println!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
            process::exit(1);
        }
    );
    println!("검색어: {}\n파일 이름: {}", config.query, config.filename);
    
    if let Err(e) = run(config) {
        println!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}
```
- 위의 run 함수를 처리하는 main 함수

#### **🤔 코드를 라이브러리 크레이트로 분리하기**

```rust
// src/lib.rs
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let c = std::fs::read_to_string(config.filename)?;
    println!("파일 내용:\n{c}");
    Ok(())
}
```

```rust
// src/main.rs
use std::{env, process};
use minigrep::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(
        |err| {
            println!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
            process::exit(1);
        }
    );
    println!("검색어: {}\n파일 이름: {}", config.query, config.filename);
    
    if let Err(e) = minigrep::run(config) {
        println!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}
```

<br>

### **4️⃣ 테스트 주도 방법으로 라이브러리의 기능 개발하기**
- `테스트 주도 개발(TDD; Test-Driven Development)`
  - 실패하는 테스트를 작성한 다음 의도한 이유 때문에 실패하는지 확인
  - 테스트에 성공하기에 충분한 코드를 작성하거나 수정
  - 추가했거나 수정한 코드를 리팩토링하면서 테스트가 계속 성공하는지 확인
  - 앞 과정을 계속 반복

#### **🤔 실패하는 테스트 작성하기**

```rust
// src/lib.rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
```
- 인수의 수명이 리턴값의 수명과 연결되었음
  - 리턴되는 벡터는 contents 인수로 전달된 문자열 슬라이스를 참조하는 슬라이스를 포함하고 있어야 함을 의미함
- **_러스트에게 search 함수가 리턴하는 데이터는 search 함수의 contents 인수로 전달된 데이터와 같은 수명을 가져야 한다는 것을 알려줌_**

```bash
running 1 test
thread 'test::one_result' panicked at 'assertion failed: `(left == right)`
  left: `["safe, fast, productive."]`,
 right: `[]`', src/lib.rs:42:9

...

 test test::one_result ... FAILED

failures:

failures:
    test::one_result

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass '-p minigrep --lib'
```

#### **🤔 테스트가 성공하도록 코드를 작성하기**

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v = vec![];
    for l in contents.lines() {
        if l.contains(query) {
            v.push(l);
        }
    }
    v
}
```

```bash
running 1 test
test test::one_result ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```rust
// src/lib.rs
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = std::fs::File::open(config.filename)?;
    let mut c = String::new();
    f.read_to_string(&mut c)?;
    
    for l in search(&config.query, &c) {
        println!("{l}");
    }

    Ok(())
}
```
- search 함수를 활용하는 run 함수

```bash
Input:
    cargo run frog poem.txt
Output:
    검색어: frog
    파일 이름: poem.txt
    How public, like a frog
```

```bash
Input:
    cargo run body poem.txt
Output:
    검색어: body
    파일 이름: poem.txt
    I'm nobody! Who are you?
    Are you nobody, too?
    How dreary to be somebody!
```

```bash
Input: 
    cargo run hihi poem.txt
Ounput:
    검색어: hihi
    파일 이름: poem.txt
```

<br>

### **5️⃣ 환경 변수 다루기**
- 사용자가 환경 변수를 이용해 문자열 검색에 대소문자를 구분하지 않도록 설정할 수 있는 기능

#### **🤔 테스트 작성하기**

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v = vec![];
    for l in contents.lines() {
        if l.contains(query) {
            v.push(l);
        }
    }
    v
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut v = vec![];
    for l in contents.lines() {
        if l.to_lowercase().contains(&query) {
            v.push(l);
        }
    }
    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let q = "duct";
        let c = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(
            vec!["safe, fast, productive."],
            search(q, c)
        );
    }

    #[test]
    fn case_insensitive() {
        let q = "rUsT";
        let c = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(q, c)
        );
    }
}
```

```bash
running 2 tests
test test::case_insensitive ... ok
test test::case_sensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### **🤔 로직 수정하기**

```rust
// src/lib.rs
use std::error::Error;
use std::io::Read;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err()
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = std::fs::File::open(config.filename)?;
    let mut c = String::new();
    f.read_to_string(&mut c)?;

    let r = if config.case_sensitive {
        search(&config.query, &c)
    } else {
        search_case_insensitive(&config.query, &c)
    };
    
    for l in r {
        println!("{l}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v = vec![];
    for l in contents.lines() {
        if l.contains(query) {
            v.push(l);
        }
    }
    v
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut v = vec![];
    for l in contents.lines() {
        if l.to_lowercase().contains(&query) {
            v.push(l);
        }
    }
    v
}
```
- `std::env` : 환경 변수를 다루는 `var` 함수를 제공하는 모듈
  - 환경 변수가 설정되어 있으면 읽어온 환경 변수 값을 저장한 Ok 값을 리턴하고, 환경 변수가 설정되어 있지 않으면 Err 값을 리턴하는 Result 타입
  - Result 값이 리턴되면 is_err 메서드를 이용해 리턴된 값이 에러인지 확인

```bash
Input:
    cargo run to poem.txt
Output:
    검색어: to
    파일 이름: poem.txt
    Are you nobody, too?
    How dreary to be somebody!
```

```bash
Input:
    export CASE_INSENSITIVE=1
    cargo run to poem.txt
Output:
    검색어: to
    파일 이름: poem.txt
    Are you nobody, too?
    How dreary to be somebody!
    To tell your name the livelong day
    To an admiring bog!
```

<br>

### **6️⃣ stderr을 이용해 에러 메시지 출력하기**
- `println!` 매크로는 표준 출력에만 지정된 메시지를 출력하므로 표준 에러를 이용해 메시지를 출력하려면 다른 방법을 사용해야 함

#### **🤔 에러의 기록 여부 확인하기**
- 표준 에러 스트림은 파일로 리다이렉트하지 않으므로 표준 에러에 출력된 메시지는 화면을 통해 확인할 수 있음
- 표준 출력 스트림을 파일로 리다이렉트한 상황에서도 에러 메시지는 표준 에러 스트림에 출력해서 화면으로 메시지를 확인할 수 있도록 구현

```bash
cargo run > output.txt
```

```
// output.txt
인수를 구문분석하는 동안 오류가 발생했습니다: 필요한 인수가 지정되지 않았습니다.
```

#### **🤔 에러를 stderr에 출력하기**

```rust
use std::{env, process};
use minigrep::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(
        |err| {
            eprintln!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
            process::exit(1);
        }
    );
    println!("검색어: {}\n파일 이름: {}", config.query, config.filename);
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}
```
- 표준 에러 스트림에 메시지를 출력하는 `eprintln!` 매크로 사용
- 프로그램의 실행 성공 메시지는 표준 출력으로 출력하고 에러 메시지는 표준 에러에 출력

<br>

## **Summary**
- 명령줄 인수, 파일, 환경 변수, 그리고 에러의 출력을 위한 `eprintln!` 매크로 등은 명령줄 애플리케이션을 개발하기 위한 기본적인 도구들