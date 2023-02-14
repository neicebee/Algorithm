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

<br>

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

<br>

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

<br>

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
- 절대 경로와 상대 경로 중 어떤 것을 사용할 것인지는 **_'아이템을 정의하는 코드를 별도로 관리할 것인지, 아니면 함께 관리할 것인지'_** 에 따라 갈림
- 해당 코드는 **_접근성 문제_** 로 컴파일되지 않기에 오작성된 코드임

#### **🤔 `pub` keyword로 경로 공개하기**
- `접근성(privacy)` : 외부 코드가 알 수 없고, 호출할 수 없고, 의존할 수 없는 상세 구현을 캡슐화하는 방법
  - 함수나 구조체를 비공개로 정의하려면 모듈 내에 정의
- 러스트의 모든 아이템(함수, 메서드, 구조체, 열거자, 모듈, 상수 등)은 기본적으로 **비공개**
  - 부모 모듈의 아이템들은 자식 모듈 안의 비공개 아이템을 사용할 수 없지만, 자식 모듈의 아이템은 부모 모듈의 아이템을 사용할 수 있음
    - 자식 모듈은 자신의 상세 구현을 감싸 숨기는 방면, 자식 모듈은 부모 모듈의 아이템이 정의된 컨텍스트를 볼 수 있음
- `pub` keyword를 이용하면 자식 모듈의 일정 부분을 외부의 부모 모듈에 공개할 수 있음

```rust
// src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}
```
- 모듈의 `pub` keyword는 부모 모듈이 하위 모듈을 참조할 수 있게만 할 뿐임
- 접근성 규칙은 구조체, 열거자, 함수, 메서드는 물론 모듈에도 적용됨

#### **🤔 `super` 로 시작하는 상대 경로**
- 상대 경로는 `super` keyword를 이용해 부모 모듈부터 시작할 수 있음
  - 파일 시스템 경로에서 `..` 문법을 이용하는 것과 같음

```rust
// src/lib.rs
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```
- `fix_incorrect_order` 함수는 `back_of_house` 모듈에 정의되어 있음
- `super` keyword는 `back_of_house` 모듈의 부모 모듈, 즉 루트 모듈인 `crate` 에 접근하여 `serve_order` 함수를 찾을 수 있음
- `super` keyword를 이용하면 나중에 코드를 다른 모듈로 이동해도 수정해야 할 코드를 최소화할 수 있음

#### **🤔 구조체와 열거자 공개하기**
- 구조체를 정의할 때 `pub` keyword를 사용한다면 구조체는 공개되지만, 구조체의 필드는 비공개임
  - 필요에 따라 각 필드를 공개하거나 비공개로 유지

```rust
// src/lib.rs
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("사과"),
            }
        }
    }
}

fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    meal.toast = String::from("밀빵");
    println!("Give me {} toast, please.", meal.toast);

    // 주석 제거 후 컴파일 시 에러 발생
    // meal.seasonal_fruit = String::from("딸기");
}
```
- `back_of_house::Breakfast` 구조체의 `toast` 필드는 공개이므로 `eat_at_restaurant` 함수가 `.` 을 이용해 `toast` 값을 읽고 쓸 수 있음
- `seasonal_fruit` 필드는 비공개이므로 `eat_at_restaurant` 함수가 접근할 수 없음
- `back_of_house::Breakfast` 구조체는 비공개 필드를 가지므로 구조체의 인스턴스를 생성할 수 있는 공개용 연관 함수를 제공해야 함
  - 만일 연관 함수가 제공되지 않는다면 비공개 필드의 값을 설정할 수 없으므로 구조체의 인스턴스를 생성할 수 없음

```rust
// src/lib.rs
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```
- 열거자는 공개하면 모든 열것값 또한 공개됨
  - 모든 열것값이 공개되지 않으면 열거자를 공개하는 의미가 없기 때문

<br>

### **4️⃣ `use` keyword로 경로를 범위로 가져오기**
- `use` keyword로 경로를 현재 범위로 가져오면 경로의 아이템이 마치 현재 범위의 아이템인 것처럼 호출할 수 있음

```rust
// src/lib.rs
mod first_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::first_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
- `use` keyword와 경로를 추가하는 것은 파일 시스템에서 `심볼릭 링크(symbolic link)` 를 생성하는 것과 유사함
- `use` keyword를 이용해 범위로 가져온 경로도 다른 경로와 마찬가지로 접근성 검사를 실행함

```rust
// src/lib.rs
mod first_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use self::first_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
- `use` keyword에 상대 경로를 지정하는 것은 현재 범위의 이름부터 시작하는 대신 `self` keyword를 이용한 경로를 사용해야 함

#### **🤔 관용적인 경로 사용하기**
- `use self::first_of_house::hosting;`
  - 관용적임
- `use self::first_of_house::hosting::add_to_waitlist;`
  - 관용적이지 않음
- 함수의 부모 모듈을 범위로 가져온 후 부모 모듈의 이름과 함수의 이름을 조합해서 호출하면 함수 경로의 반복을 최소화하면서도 이 함수가 로컬에 정의된 것이 아니라는 점을 더 명확히 할 수 있음

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```
- 구조체, 열거자, 혹은 기타 다른 아이템을 `use` 구문으로 가져올 때는 전체 경로를 사용하는 것이 관용적임

```rust
use std::io; // => fmt::Result
use std::fmt; // => io::Result<()>
```
- `use` 구문을 이용해 같은 이름을 가진 두 아이템을 현재 범위로 가져오는 것은 러스트가 지원하지 않음
- 해당 코드처럼 부모 모듈을 사용하면 두 `Result` 타입을 구분할 수 있음
- `use std::io::Result` 와 `use std::fmt::Result` 구문을 사용했다면 두 `Result` 타입이 같은 범위에 있기 때문에 어느 부모 모듈에 정의된 것을 가리키는지 이해할 수 없음

#### **🤔 `as` keyword로 새로운 이름 부여하기**
- `use` 구문으로 같은 이름을 가진 두 타입을 현재 범위로 가져오려면 경로 뒤에 `as` keyword를 이용해 해당 타입에 새로운 이름을 부여하면 됨

```rust
use std::io::Result;
use std::fmt::Result as FmtResult;
```
- `std::fmt::Result` 타입에 `FmtResult` 라는 이름을 부여함으로써 이름 충돌 없이 범위로 가져옴

#### **🤔 `pub use` keyword로 이름을 다시 내보내기**
- `use` keyword를 이용해 범위로 이름을 가져오면 이 이름은 새 범위에서 비공개 이름이 됨
- 호출하는 코드가 다른 범위에서 가져온 이름도 현재 범위에 정의된 것처럼 접근할 수 있도록 하고자 한다면 `pub` 와 `use` keyword를 조합하면 됨
  - `다시 내보내기 (re-exporting)`
- `다시 내보내기` 는 다른 개발자가 생각하는 코드의 호출 방식과 실제 코드의 내부 구조가 다를 때 유용함
  - 코드의 내부 구조를 유지하면서 외부에는 다른 구조로 코드를 노출할 수 있음
  - 라이브러리를 작업하는 프로그래머 입장에서 원하는 구조를 유지하면서도 라이브러리를 호출하는 프로그래머에게 편리한 구조를 노출할 수 있음

```rust
// src/lib.rs
mod first_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::first_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
- `pub use` 구문을 사용하지 않으면 `eat_at_restaurant` 코드는 `hosting::add_to_waitlist` 함수를 호출할 수 있지만, 외부의 코드는 이 함수를 호출할 수 없음

#### **🤔 외부 패키지의 사용**
- `https://crates.io/` 에는 러스트 커뮤니티 구성원들이 만들어 둔 많은 패키지가 등록되어 있음
- 패키지의 `Cargo.toml` 파일에 필요한 크레이트를 나열하고 `use` 구문을 이용해 범위로 가져오면 됨
- `표준 라이브러리(std)` 또한 외부 크레이트임
  - `표준 라이브러리` 는 러스트 언어와 함께 제공되기에 크레이트를 추가할 필요가 없음
  - 하지만 패키지의 범위로 가져오려면 크레이트를 참조해야 함

```rust
// Cargo.toml
[package]
name = "restaurant"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.5.5"
```
- `Cargo.toml` 파일에 `rand` 를 의존성으로 추가
  - 카고는 이 패키지와 `rand` 패키지를 사용하기 위한 다른 모든 의존성 패키지를 `https://crates.io/` 에서 내려받음

#### **🤔 중첩 경로로 `use` 목록을 깔끔하게 유지하기**

```rust
use std::io;
use std::cmp::Ordering;
```
- 중첩 경로를 적용하지 않은 `use` 구문

```rust
use std::{io, cmp::Ordering};
```
- 중첩 경로를 적용한 `use` 구문
- 공통 부분을 먼저 기술한 다음 중괄호를 이용해 각기 다른 경로를 작성하면 됨

```rust
use std::io;
use std::io::Write;
```
- 한 경로의 하위 경로를 가져오는 `use` 구문

```rust
use std::io::{self, Write};
```
- 중첩 경로를 적용한 `use` 구문

#### **🤔 글롭 연산자**
- 어떤 경로의 공개 아이템을 모두 현재 범위로 가져오려면 글롭 연산자인 `*` 을 사용해 경로를 지정

```rust
use std::collections::*;
```
- `std::collections` 모듈에 정의된 모든 공개 아이템을 현재 범위로 가져오는 `use` 구문
- 글롭 연산자는 주의해서 사용해야 함
  - 범위에 어떤 이름을 가져왔는지, 그 이름이 프로그램 어디에 정의되어 있는지 알기가 어렵기 때문

<br>

### **5️⃣ 모듈을 다른 파일로 분리하기**
- 모듈의 크기가 커지면 코드를 더 쉽게 탐색하기 위해 별도의 파일로 분리하는 것이 효과적임

```rust
// src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
- `front_of_house` 모듈 선언

```rust
// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```
- `front_of_house` 모듈 정의
- 모듈 선언 시 코드 블록 대신 세미콜론을 추가하면 러스트는 해당 모듈의 콘텐츠를 모듈과 같은 이름의 파일에서 가져옴
  - `hosting` 모듈의 콘텐츠를 다른 파일로 옯기면 `front_of_house.rs` 파일에도 `hosting` 모듈을 선언하는 부분만 남게 됨

```rust
// src/front_of_house.rs
pub mod hosting;
```

```rust
// src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```
- 이런 식으로 분리해도 모듈 트리는 같은 형태로 유지됨
- `use` 구문은 크레이트의 일부로 컴파일되는 파일의 위치가 변경되어도 아무런 영향을 받지 않음
- `mod` keyword는 모듈을 선언하며, 러스트는 모듈과 같은 이름의 파일에서 모듈의 콘텐츠를 가져오게 됨

<br>

## **Summary**
- 러스트에서는 패키지를 크레이트로 정리하며 크레이트는 모듈을 구성함
  - 한 모듈에 정의된 아이템을 다른 모듈에서 참조할 수 있음
- 아이템을 참조하려면 절대 혹은 상대 경로를 사용
  - 이 경로는 `use` 구문을 이용해 지정된 아이템을 현재 범위로 가져오면 여러 아이템을 현재 범위로 가져올 때 더 짧은 형식의 구문을 사용할 수도 있음
- 모듈의 코드는 기본적으로 비공개이지만, `pub` keyword로 아이템을 공개할 수 있음