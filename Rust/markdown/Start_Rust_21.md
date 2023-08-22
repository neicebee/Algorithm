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