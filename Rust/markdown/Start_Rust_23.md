# 🦀 Rust Day 23

## **🏳️ More About Cargo and Crates.io**

### **1️⃣ 릴리즈 프로필을 이용한 빌드 커스터마이징**
- 개발자들이 코드의 컴파일을 더 상세히 제어할 수 있도록 다양한 설정의 커스터마이징이 가능한 프로필이 준비되어 있음
  - 각 프로필은 서로 독립적 구성
- 카고에는 두 개의 주 프로필
  - cargo build : **dev 프로필**
    - 개발 과정에 적합한 기본 설정을 갖춤
  - cargo build --release : **release 프로필**
    - 릴리즈용 빌드를 위한 기본 설정을 갖춤

```rust
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```
- dev와 release 프로필의 기본 opt-level 설정
  - 러스트가 코드에 적용할 최적화 수준을 지정하며 유효한 범위는 0부터 3까지
    - 더 많은 최적화를 제공할수록 컴파일 시간이 늘어남
    - 개발 환경에서 코드를 자주 컴파일할 때는 결과 코드가 다소 느리게 실행되더라도 컴파일이 빠른 편이 도움이 됨
    - 때문에 dev 프로필의 opt-level 설정 기본값은 0
    - 릴리즈를 위해 코드를 빌드할 때는 컴파일에 시간을 더 할애하는 편이 나음
    - 때문에 release 프로필의 opt-level 설정 기본값은 3
- 카고는 프로젝트의 Cargo.toml 파일에 [profile.*] 섹션이 별도 설정되어 있지 않으면 각 프로필에 따라 다른 기본 설정을 적용함

<br>

### **2️⃣ crates.io 사이트에 크레이트 발행하기**

#### **🤔 유용한 문서 주석 작성하기**
- 러스트는 문서화를 위한 특별한 형식의 주석을 지원함
- **문서 주석(documentation comment)**
  - 주석 내용을 토대로 HTML 문서 생성
  - 공개 API에 작성된 문서 주석의 내용을 보여줌
  - 크레이트를 어떻게 구현했는지보다는 크레이트를 어떻게 사용해야 하는지 알고 싶어 하는 프로그래머들을 위함

```rust
/// 주어진 숫자에 1을 더한다.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```

pub fn add_one(x: i32) -> i32 {
    x+1
}
```
- 함수의 문서 주석
  - 문서 주석은 슬래시 세 개로 시작
  - 텍스트 형식화를 위한 markdown 문법 지원
  - 주석 작성 아이템 바로 전에 작성하면 됨
  - cargo doc 명령 실행 시 해당 주석을 이용해 HTML 문서를 생성할 수 있음
    - `cargo doc --open` : 현재 크레이트의 문서 주석으로 생성한 HTML 문서 생성 후 브라우저를 통해 보여줌

**(1)** 문서 주석에 사용하는 섹션
- Panics
  - 함수가 패닉을 발생시키는 경우를 설명함
  - 이 함수의 호출자가 프로그램이 패닉을 발생하는 것을 원치 않을 때는 이 함수를 호출하지 않도록 주의해야 함
- Errors
  - 함수가 Result 타입을 리턴하는 경우에는 어떤 종류의 에러가 발생할 수 있는지, 어떤 조건에서 이런 에러가 발생하는지를 명시하면 호출자가 발생 가능한 에러를 적절히 처리하는 데 도움이 됨
- Safety
  - 함수의 호출이 안전하지 않다면, 이 함수가 왜 안전하지 않으며 호출자가 함수를 호출할 때 주의해야 할 내용들을 반드시 언급해야 함

<br>

**(2)** 문서 주석을 테스트에 활용하기
- cargo test 명령 실행 시 문서의 예제 코드를 테스트로 실행해 줌

<br>

**(3)** 아이템을 보유한 루트를 위한 주석
- 아이템에 주석을 추가하는 것이 아닌 주석을 가지고 있는 아이템을 문서에 추가하려면 `//!` 사용
  - 크레이트의 루트 파일(src/lib.rs)에 작성하거나 크레이트나 모듈 전체를 문서화하기 위해 모듈 안에 작성함

```rust
//! # My Crate
//! 
//! `my crate` 는 일부 연산을 더 쉽게 실행하기 위한 유틸리티 모음이다.

```
- my_crate 전체에 대한 문서화 주석
  - 크레이트와 모듈을 설명하는 데 유용함

<br>

#### **🤔 pub use 키워드를 이용해 공개 API 발행하기**
- 공개 API 구조는 크레이트를 발행할 때 가장 주의해야 할 부분
- pub use 키워드를 이용하면 내부 구조와는 다른 공개용 구조로 아이템을 다시 노출(re-exporting)할 수 있음

```rust
// src/lib.rs

//! # Art
//! 
//! 미술품을 모델링하기 위한 라이브러리

pub mod kinds {
    /// RYB 색상 모델에 따른 주 색상
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// RYB 색상 모델에 따른 보조 색상
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// 두 개의 주 색상을 조합해서
    /// 보조 색상을 생성한다.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // -- 생략 --
    }
}
```
- kinds와 utils 모듈로 구성된 art 라이브러리
- 이 라이브러리를 참조하는 다른 크레이트는 use 구문을 이용해 art 라이브러리를 현재 범위로 가져온 후, 라이브러리에 정의된 모듈 구조에 따라 아이템을 사용해야 함

```rust
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```
- art 라이브러리의 내부 구조에 따라 필요한 항목 사용
- kinds 모듈과 utils 모듈에 나누어 구성된 내부 구조는 art 라이브러리를 사용하려는 누군가에게는 그다지 유용한 정보가 아님
  - 사용자가 직접 필요한 아이템의 위치를 찾아야 하므로 모듈 구조는 혼란을 가중시킬 뿐만 아니라, use 구문에 모듈의 이름을 모두 명시해야 하므로 불편하기도 함

```rust
//! # Art
//! 
//! 미술품을 모델링하기 위한 라이브러리

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;
```
- pub use 구문을 이용해 아이템을 최상위 수준으로 다시 노출

```rust
use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
```
- 다시 노출된 아이템을 사용하는 프로그램
  - 중첩 모듈이 많을 때 pub use 구문을 이용해 타입을 최상위로 다시 노출하면 크레이트를 사용하는 사람들에게 훨씬 편리한 경험을 제공할 수 있음
  - pub use 구문 사용 시 크레이트의 내부 구조를 마음대로 구성할 수 있는 유연성을 제공하면서도 내부 구조를 실제 사용자에게 노출하지 않음

<br>

#### **🤔 crate.io 계정 생성하기**
- `cargo login [API 키]`
  - ~/.cargo/credentials 파일에 저장

<br>

#### **🤔 새 크레이트의 메타데이터 추가하기**
- 발행에 앞서 Cargo.toml 파일에 [package] 섹션을 추가해서 크레이트에 대한 메타데이터를 제공해야 함

```rust
[package]
name = "guessing_game"
```
- 크레이트는 유일한 이름이 필요함
  - 로컬에서 개발 중이라면 원하는 이름을 부여해도 됨
  - crates.io에서 등록할 이름이 선점되지 않았다면 Cargo.toml 파일의 [package] 섹션에 발행할 이름을 지정하면 됨

