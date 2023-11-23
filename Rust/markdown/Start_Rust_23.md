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