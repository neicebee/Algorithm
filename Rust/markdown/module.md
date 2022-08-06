# 💻 Rust Module

- `fn`: function
- `pub`: public
- `mod`: module

## 💡 Main 에서의 Import

- Rust 프로젝트 디렉터리에 포함되어 있는 src 디렉터리에 code1 소스코드를 작성하고 Import
  - **code1**
    ```rust
    pub fn code1(){
        println!("import1!");
    }
    ```
    - `pub fn code1`: code1 함수 공개
  - **main**
    ```rust
    mod code1;

    fn main(){
        code1::code1();
    }
    ```
    - `mod code1;`: 파이썬의 import

## 💡 Sub 에서의 Import

- Rust 프로젝트 디렉터리에 하위 디렉터리 algo를 생성 후 해당 디렉터리 내에 code2와 code3 소스코드를 작성하고 Import
  - <u>*서브 디렉터리 내의 소스를 가져오려면 그 디렉터리에 반드시 `mod.rs` 파일이 있어야 함. 디렉터리 명으로 모듈을 가져오려고 시도하면, 반드시 그 안의 `mod.rs`를 가져오려고 하기 때문*</u>
  - **code2**
    ```rust
    pub fn code2(){
        println!("import1!");
    }
    ```
  - **code3**
    ```rust
    pub fn code3(){
        println!("import1!");
    }
    ```
  - **mod**
    ```rust
    pub mod code2;
    pub use code2::*;
    pub mod code3;
    pub use code3::*;
    ```
  - **main** 
    ```rust
    mod algo;

    fn main(){
        algo::code2();
        algo::code3();
    }
    ```