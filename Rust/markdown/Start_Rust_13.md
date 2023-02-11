# 🦀 Rust Day 13

## **🏳️ Managing Growing Projects with Packages, Crates, and Modules**
- 프로젝트가 커지면 코드를 여러 개의 파일과 모듈로 나누어 관리하는 편이 효율적임
- 패키지는 여러 개의 바이너리 크레이트를 포함할 수 있으며, 필요하다면 하나의 라이브러리 크레이트도 추가할 수 있음
- 패키지의 크기가 커지면 일정 부분을 다른 크레이트로 옮겨 외부 의존성으로 추가할 수 있음
- 구현을 캡슐화하면 코드를 재사용할 수 있음
- `Module System` : 러스트 코드의 구조 관리
  - `Package` : 크레이트를 빌드, 테스트, 공유할 수 있는 카고의 기능
  - `Crate` : 라이브러리나 실행 파일을 생성하는 모듈의 트리(tree)
  - `Module & use` : 코드의 구조와 범위, 그리고 경로의 접근성을 제어하는 기능
  - `Path` : 구조체, 함수, 혹은 모듈 등의 이름을 결정하는 방식

### **1️⃣ 패키지와 크레이트**
- `Crate`
  - 하나의 바이너리 혹은 라이브러리
  - `Crate root` 는 러스트 컴파일러가 컴파일을 시작해서 크레이트의 루트 모듈을 만들어 내는 소스 파일
- `Package`
  - 일련의 기능을 제공하는 하나, 혹은 그 이상의 크레이트로 구성됨
  - 패키지는 이 크레이트를 빌드하는 방법을 서술하는 `Cargo.toml` 파일을 가짐
- 패키지는 하나 혹은 그 이상의 라이브러리 크레이트를 포함하거나 아예 포함하지 않을 수 있음
  - 바이너리 크레이트도 원하는 만큼 포함할 수 있지만, 최소한 하나의 크레이트는 포함해야 함
- 💀 `cargo new project_1`
  - **(1)** 카고가 `Cargo.toml` 파일을 생성해 패키지를 만들어 냄
  - **(2)** `Cargo.toml` 파일에는 `src/main.rs` 파일에 대한 언급이 없음
    - `src/main.rs` 파일은 패키지와 같은 이름을 갖는 바이너리 크레이트의 크레이트 루트임
    - 카고는 패키지 디렉터리에 `src/lib.rs` 파일이 있으면 이 패키지는 패키지와 같은 이름의 라이브러리 크레이트를 포함한다고 판단하며, `src/lib.rs` 파일을 크레이트 루트로 인식함
    - 카고는 라이브러리나 바이너리를 빌드할 때 `rustc` 컴파일러에게 크레이트 루트 파일을 전달함
  - **(3)** `project_1` 패키지에는 `src/main.rs` 파일만 있음
    - `project_1` 이라는 이름의 바이너리 크레이트를 포함한다는 의미
    - 만약 `src/main.rs` 파일과 `src/lib.rs` 파일을 모두 가진다면 라이브러리와 바이너리 크레이트를 모두 가진다는 의미이며, 두 크레이트 모두 이름이 패키지 이름과 같음
  - **(4)** 패키지의 `src/bin` 디렉터리에 파일을 분산하여 여러 개의 바이너리 크레이트를 추가할 수 있음
    - 이때 각 디렉터리의 파일들은 별개의 바이너리 크레이트가 됨
- 크레이트는 관련된 기능들을 하나의 범위로 그룹화하므로 해당 기능을 여러 프로젝트에서 공유하기 수월해짐
- 크레이트의 기능을 해당 크레이트의 범위 안에서 구현하면 특정 기능이 작성 중인 크레이트에 정의된 것인지, 아니면 추가한 크레이트에 정의된 것인지 명확히 구분할 수 있음
  - **(Ex)**
    - `Rng` : 작성 중인 크레이트에서 정의한 구조체
    - `rand::Rng` : `rand` 크레이트의 `Rng` 트레이트

### **2️⃣ 모듈을 이용한 범위와 접근성 제어**
- `Path` : 아이템의 이름을 결정
- `use` keyword : `Path` 를 범위 안으로 가져옴
- `pub` keyword : 아이템을 외부에 공개함
- `Module`
  - 크레이트의 코드를 그룹화해서 가독성과 재사용성을 향상하는 방법
  - 아이템의 `접근성(privacy)` 결정

```rust
// src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
```
- 레스토랑의 기능을 제공하는 간단한 바이너리 크레이트 예제
- `src/lib.rs` 파일에 모듈과 함수 시그니처 정의
- `mod` keyword로 모듈 정의
- 모듈에는 구조체, 열거자, 상수, 트레이트, 함수 등을 추가할 수 있음
- 모듈을 이용하면 관련된 정의들을 하나의 그룹으로 묶어 적절한 이름을 부여할 수 있음

```bash
crate
└── front_of_house
    ├── hosting
    │   ├── add_to_waitlist
    │   └── seat_at_table
    └── serving
        ├── take_order
        ├── serve_order
        └── take_payment
```
- `모듈 트리(module tree)`
- 컴퓨터 파일 시스템의 디렉터리 트리와 유사함

### **3️⃣ 경로를 이용해 모듈 트리의 아이템 참조하기**
- `절대 경로(absolute path)` : 크레이트 이름이나 `crate` 리터럴을 이용해 크레이트 루트부터 시작하는 경로
- `상대 경로(relative path)` : 현재 모듈로부터 시작하며, `self`, `super` 혹은 현재 모듈의 식별자를 이용함
- 절대 및 상대 경로는 하나 혹은 그 이상의 식별자로 구성되며 각 식별자는 `이중 콜론(::)` 으로 구분함

```rust
// src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}
```
- `add_to_waitlist` 함수는 `eat_at_restaurant` 함수와 같은 크레이트에 정의되어 있으므로 절대 경로를 `crate` keyword로 시작할 수 있음
- 모듈 트리에서 `eat_at_restaurant` 함수가 정의된 모듈과 같은 수준의 모듈인 `front_of_house` 부터 상대 경로 시작