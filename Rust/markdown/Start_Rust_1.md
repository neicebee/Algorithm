# 🦀 Rust Day 1

![Rust_book](https://w.namu.la/s/93e2abc7d38a7aa68fc6e8af65e5719040f67b6640b87efe5d304d4d27af3b4d149e5acc6b854d37974f816de9a6694510278cce5c2fa47edae024c6377d5eeed0ee50a7c9d65f8ceaeca30bc587a6a6254bdf4f2dbdc0dab375ed2a28caa6e7b4dfaa9dbbff6feea3838eb7af328947)

## 🏳️ Rust

Rust는 2006년 Mozilla 연구소의 Graydon Hoare의 설계로부터 시작되었다.

Rust는 Low 레벨의 시스템 프로그래밍 언어이며, Static typing을 사용하여 컴파일 타임에 다양한 오류를 감지함.

즉, OS 개발과 시스템의 개념을 이해하고자 하는 사람에게는 공부하기 좋은 언어이다.

하지만 진입 장벽이 낮지는 않다고 한다.

## 🏴 Code

```rust
fn main(){
    println!("Hello, World!");
}
```

- C와 같이 main 함수는 실행 가능한 모든 Rust 프로그램에서 가장 첫 번째로 실행된다.
- Rust에서의 들여쓰기는 탭이 아닌 공백 문자 4개를 이용한다.
- `println()`: 함수
- `println!()`: 매크로

## 🏴‍☠️ Static Language

- 정적 언어인 Rust는 컴파일과 실행을 독립적인 단계로 구분한다.
  - Low Level Language
  - 프로그램을 컴파일해서 생성된 바이너리를 타인에게 전달하면 해당 언어를 설치하지 않고도 실행이 가능
- Ruby, Python, Javascript는 동적 언어로 컴파일과 실행을 하나의 명령으로 처리한다
  - High Level Language
  - 언어당사자가 해당 언어를 설치해야지 실행이 가능

## 🏁 Cargo

Rust의 빌드 시스템이자 패키지 관리자

- `cargo new [project_name]`: 프로젝트 생성
  - 해당 프로젝트 디렉터리가 깃 저장소로 초기화
- `Cargo.toml`: TOML(Tom's Obivous, Minimal Language) 형식, 해당 형식은 카고의 설정 파일 형식
- `cargo build`: 프로젝트 빌드. target/debug/[project_name] 경로에 실행 파일이 저장됨
- `cargo run`: 프로젝트 컴파일 및 결과 파일 실행
- `cargo check`: 코드의 컴파일 여부를 신속하게 검사. 실행 파일 생성하지 않음
- `cargo build --release`: 최적화된 컴파일
  - `cargo build`와 다르게 /target/release 경로에 실행 파일 생성
  - `cargo build`는 더 빨리 자주 컴파일하기 위함
  - `cargo build --release`는 최종 완성된 프로그램을 사용자에게 제공하기 위해 최대한 빠르게 실행될 수 있도록 컴파일하기 위함