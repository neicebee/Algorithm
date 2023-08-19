# 🦀 Rust Day 20

## **🏳️ Writing Automated Tests**
- 러스트는 자동화된 소프트웨어 테스트의 작성을 언어 차원에서 지원함
- 개발자는 함수에 값을 전달하고 결과값이 잘 리턴되는지 확인하는 테스트를 작성할 수 있음
  - 코드를 수정할 때마다 이 테스트를 실행하여 원래 잘 동작하던 기능이 여전히 잘 작동하는지 확인할 수 있음

<br>

### **1️⃣ 테스트의 작성**
- 테스트 함수의 본문은 다음 세 가지 동작을 수행함
  - **1** 필요한 데이터나 상태를 설정
  - **2** 테스트할 코드를 실행
  - **3** 의도한 결과가 출력되는지 검증

#### **🤔 테스트 함수 살펴보기**
- 테스트는 `test` 특성을 적용한 러스트 함수
  - `특성(attributes)` : 러스트 코드에 대한 메타데이터
    - (Ex) `derive` 특성
- 함수를 테스트 함수로 전환하려면 `fn` 키워드를 사용한 코드 위에 `#[test]` 특성을 적용하면 됨
- `cargo test` 명령을 이용해 테스트를 실행하면 러스트는 `test` 특성이 적용된 함수들을 실행하는 `테스트 실행기(test runner)` 바이너리를 빌드하고 각 테스트의 성공 여부를 보고함

```rust
// src/lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```
- `cargo new` 명령이 자동으로 생성한 테스트 모듈과 함수
- `#[test]` : 함수가 테스트 함수임을 가리키며 테스트 실행기는 이 특성을 이용해 어떤 함수가 테스트 함수인지를 판단함
  - 테스트 모듈 내에서 공통의 시나리오나 작업을 수행할 일반 함수를 작성할 수 있으므로 해당 특성을 이용해 어떤 함수가 테스트 함수인지를 반드시 표기해야 함
- `cargo test` : 프로젝트 안의 모든 테스트를 실행하는 명령

```bash
running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
- `passed & failed` : 성공한 테스트 및 실패한 테스트 개수
- `ignored` : 무시하도록 표시된 테스트
- `filtered out` : 테스트의 실행에 필터 적용 유무
- `measured` : 성능 측정하는 벤치마크 테스트를 위한 것
- `Doc-tests` : 문서 테스트 결과

```rust
// src/lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2+2, 4);
    }

    #[test]
    fn another() {
        panic!("failed...");
    }
}
```
- 테스트는 테스트 함수의 어딘가에서 패닉이 발생하면 실패함
  - 각 테스트는 새로운 스레드에서 실행되며 주 스레드는 어떤 테스트 스레드가 중단되는 것을 확인하면 해당 테스트는 실패한 것으로 간주함

```bash
running 2 tests
test tests::exploration ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'failed...', src/lib.rs:10:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

#### **🤔 `assert!` 매크로를 이용해 결과 확인하기**
- `assert!` : 테스트의 성공 여부를 평가할 때 유용하게 활용하는 매크로
  - 불리언으로 평가되는 표현식을 인수로 전달하면 됨
  - 해당 매크로에 `true` 를 전달하면 별다른 추가 동작 없이 테스트 성공
  - 전달된 값이 `false` 일 경우 패닉이 발생하여 테스트에 실패함

```rust
// src/lib.rs
#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn larger_cannot_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(!smaller.can_hold(&larger));
    }
}
```
- `use super::*;` : `tests` 모듈은 중첩된 모듈이므로 바깥 모듈의 테스트 코드를 중첩된 모듈의 범위로 옮겨와야 하기 때문에 `*` 기호를 사용하여 바깥 모듈에 정의한 모든 타입을 범위로 가져옴

```bash
running 2 tests
test tests::larger_can_hold_smaller ... ok
test tests::larger_cannot_hold_smaller ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### **🤔 `assert_eq!` 와 `assert_ne!` 매크로를 이용한 동등 비교 테스트**
- `assert_eq! & assert_ne!` : 각각 두 개의 인수가 같은지 혹은 다른지를 비교함
  - 검증이 실패하면 왜 실패했는지 쉽게 알 수 있도록 인수로 전달된 두 값을 출력해 줌
  - 내부적으로 각각 `==` 와 `!=` 연산자를 사용함
    - 매크로에 전달되는 값들은 `PartialEq` 와 `Debug` 트레이트를 구현해야 함
      - 모든 기본 자료형과 표준 라이브러리가 제공하는 타입들은 해당 트레이트를 구현하고 있음
      - 직접 선언한 구조체와 열거자는 개발자가 직접 해당 트레이트를 구현해야 동등 비교를 수행할 수 있음
      - 두 트레이트는 상속이 가능하므로 구조체나 열거자를 선언 시 `#[derive(PartialEq, Debug)]` 애노테이션을 추가해 주기만 하면 됨

```rust
// src/lib.rs
pub fn add_two(a: i32) -> i32 {
    a+2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
    }
}
```

```bash
running 1 test
test tests::it_add_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```rust
// src/lib.rs
pub fn add_two(a: i32) -> i32 {
    a+3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
    }
}
```

```bash
running 1 test
test tests::it_add_two ... FAILED

failures:

---- tests::it_add_two stdout ----
thread 'tests::it_add_two' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::it_add_two

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### **🤔 사용자 정의 실패 메시지**
- `assert! & assert_eq! & assert_ne!` 매크로의 선택형 인수를 이용하면 실패 메시지에 사용자 정의 메시지를 추가할 수 있음
  - 필수 매개변수 다음 형식화된 문자열을 전달할 수 있음

```rust
// src/lib.rs
pub fn greeting(name: &str) -> String {
    format!("Hi, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let r = greeting("화비");
        assert!(
            r.contains("히히"),
            "함수 결과에 이름이 포함되어 있지 않음.\n결괏값: {}", r
        );
    }
}
```

```bash
running 1 test
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at '함수 결과에 이름이 포함되어 있지 않음.
결괏값: Hi, 화비!', src/lib.rs:12:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### **🤔 `should_panic` 매크로를 이용해 패닉의 발생 여부 검증하기**
- `should_panic` 특성 : 함수 내의 코드가 패닉이 발생하면 테스트를 성공하고 패닉이 발생하지 않으면 실패하도록 함
  - 단순히 패닉이 발생해야 한다는 것만을 지시하므로 다소 모호함
  - 의도와는 다른 원인으로 패닉이 발생해도 성공함
  - 해당 특성이 적용된 테스트의 동작을 조금 더 정확하게 하려면 선택형 매개변수를 추가하면 됨

```rust
// src/lib.rs
pub struct Guess {
    v: u32,
}

impl Guess {
    pub fn new(v: u32) -> Guess {
        if v < 1 {
            panic!(
                "반드시 1보다 크거나 같은 값 사용. 지정값: {}", v
            );
        } else if v > 100 {
            panic!(
                "반드시 100보다 작거나 같은 값 사용. 지정값: {}", v
            );
        }
        
        Guess { v }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "반드시 100보다 작거나 같은 값 사용.")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```
- `Guess::new` 함수가 패닉을 발생할 때 함께 출력할 메시지의 일부를 검사하므로 무사히 성공함
- 해당 특성의 `expected` 메시지가 일치하지 않으면 테스트가 실패함

```bash
running 1 test
test tests::greater_than_100 - should panic ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

<br>

### **2️⃣ 테스트 실행 제어하기**
- `cargo test` 명령의 기본 동작을 변경하기 위해 명령줄 옵션을 적용할 수 있음
  - 명령줄 옵션의 일부는 `cargo test` 명령에 적용되면 나머지는 결과 테스트 바이너리의 생성에 적용됨
  - 옵션 적용 시 이 두 종류의 옵션을 구분하기 위해 해당 명령에 적용되는 인수들을 나열하고 테스트 바이너리에 적용되는 옵션들은 구분자 `--` 다음에 나열하면 됨

#### **🤔 테스트를 병렬 혹은 직렬로 실행하기**
- 여러 개의 테스트 실행 시 기본적으로 `스레드(thread)` 를 이용해 `병렬(parallel)` 로 실행함
  - 신속하지만 여러 테스트가 동시에 실행되므로 각 테스트는 현재 작업 디렉터리나 환경 변수 등 공통적으로 적용되는 환경을 포함한 `공유 상태(shared state)` 에 대해 독립적이어야 함
- 테스트를 병렬로 실행하기를 원치 않거나 사용할 스레드의 개수를 조금 더 정밀하게 제어하고 싶다면 `--test-threads` 플래그에 테스트 바이너리가 사용할 스레드의 숫자를 지정하면 됨
  - `cargo test -- --test-threads=1`
    - 테스트 스레드의 개수를 1로 설정
    - 하나의 스레드만을 이용해 테스트를 실행하면 병렬로 실행하는 것보다 실행 시간은 오래 걸리지만, 테스트 간의 간섭을 없앨 수 있음

#### **🤔 함수의 결과 보여주기**
- 기본적으로 테스트에 성공하면 러스트의 테스트 라이브러리는 표준 출력에 아무것도 출력하지 않음
  - 테스트 함수 내에서 `println!` 매크로를 호출해도 테스트가 성공하면 출력을 터미널에서 확인할 수 없고 단지 테스트가 성공했다는 메시지만 보임
  - 테스트 실패 시 테스트 실패 메시지와 함께 출력한 내용도 확인 가능
- 성공한 테스트의 출력값도 확인하고 싶다면 `--nocapture` 플래그를 지정해 출력 가로채기 동작을 비활성화할 수 있음
  - `cargo test -- --nocapture`

#### **🤔 이름을 이용해 테스트 일부만 실행하기**
- `cargo test` 명령에 실행하고자 하는 테스트의 이름을 인수로 전달해서 원하는 테스트만 실행할 수 있음
  - `cargo test [test_name]`
    - 하나의 테스트만 실행
  - `cargo test [keyword]`
    - 테스트 이름의 일부만 키워드로 지정 시 지정된 키워드에 일치하는 모든 테스트 실행

#### **🤔 이름을 이용해 테스트 일부만 실행하기**
- `#[test]` 다음에 `#[ignore]` 특성 적용 시 해당 테스트 함수 실행에서 제외 가능
- `cargo test -- --ignored`
  - 제외된 테스트만을 실행

<br>

### **3️⃣ 테스트의 조직화**
- `단위 테스트(unit tests)` : 작고 집중적이며 한 번에 하나의 모듈을 독립적으로 테스트하고 비공개 인터페이스를 테스트할 수도 있음
- `통합 테스트(integration tests)` : 완전히 라이브러리 이외의 것을 테스트하는 것
  - 외부 코드와 같은 방법으로 테스트 코드를 사용하며, 공개 인터페이스만을 테스트할 수 있고 하나의 테스트가 여러 개의 모듈을 활용할 수도 있음

#### **🤔 단위 테스트**
- 코드의 각 단위를 나머지 코드와는 독립적으로 테스트해서 코드가 의도대로 동작하는지를 빠르게 판단하는 것
- src 디렉터리의 각 파일에 테스트할 코드와 함께 작성
  - 각 파일에 `tests` 라는 모듈을 `cfg(test)` 라는 특성과 함께 선언하고, 그 안에 테스트 함수들을 작성

<br>

**(1)** Tests 모듈과 #[cfg(test)] 특성
- `tests` 모듈의 `#[cfg(test)]` 특성은 러스트에게 이 모듈은 `cargo test` 명령을 실행할 때만 코드를 컴파일하고 실행하라고 지시하는 것
  - 테스트 코드가 컴파일 결과에 포함되지 않도록 꼭 지정해야 함
  - 컴파일 되는 코드에는 `#[test]` 특성이 적용된 테스트 함수는 물론 모듈 내의 `도움 함수(helper functions)` 들도 포함됨
- `cfg` 특성 : `설정(configuration)` 의 약자
  - 러스트에게 이후의 코드는 특정 설정 옵션이 지정되었을 때만 포함해야 한다는 것을 알려줌

<br>

**(2)** 비공개 함수의 테스트
- 다른 언어는 비공개 함수의 테스트가 매우 어렵거나 아예 불가능함
- 러스트의 비공개 규칙은 비공개 함수의 테스트를 허용함

```rust
// src/lib.rs
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a+b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

```bash
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

<br>

#### **🤔 통합 테스트**
- 라이브러리의 영역 바깥에서 진행됨
- 다른 코드와 같은 방법으로 라이브러리 사용
  - 라이브러리의 공개 API만 호출 가능하다는 뜻
- 라이브러리의 여러 부분이 올바르게 동작하는지 확인하는 것
- 통합 테스트를 생성하려면 tests 디렉터리를 생성해야 함

<br>

**(1)** tests 디렉터리
- tests 디렉터리는 src 디렉터리처럼 최상위 수준에 생성
  - 카고는 이 디렉터리에서 통합 테스트 파일을 탐색
  - 이 디렉터리 안에 원하는 만큼의 테스트 파일을 생성
  - 각 파일을 별개의 크레이트로 컴파일

```rust
// tests/integration_test.rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```
- tests 디렉터리의 각 테스트는 별개의 크레이트로 컴파일되므로 각 파일에서 라이브러리를 가져와야 함
- `#[cfg(tests)]` 특성을 지정할 필요가 없음
  - 카고는 tests 디렉터리를 특별히 다루며, 이 디렉터리의 파일들은 `cargo test` 명령을 실행할 때만 컴파일함

```bash
running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs (target/debug/deps/integration_test-7c809cc164d12514)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
- `cargo test` 명령 실행 결과
- 단위 테스트, 통합 테스트, 문서 테스트 등 세 개의 섹션이 출력됨

```bash
running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
- `cargo test --test integration_test` 명령 실행 결과
- 통합 테스트 함수의 이름을 지정하면 특정 통합 테스트만 실행할 수 있음

<br>

**(2)** 통합 테스트의 서브 모듈
- 통합 테스트의 양이 늘어나면 tests 디렉터리에 하나 이상의 파일을 생성하는 것이 테스트를 구조적으로 유지하는 데 도움이 됨
- tests 디렉터리의 서브 디렉터리에 작성한 파일들은 별개의 크레이트로 컴파일되지 않으며 출력 결과에 별개의 섹션으로 나타나지도 않음
- 예를 들어, tests/common/mod.rs 파일을 생성한 후 통합 테스트 파일에서 모듈처럼 사용하면 됨

<br>

**(3)** 바이너리 크레이트의 단위 테스트
- 현재 진행 중인 프로젝트가 src/lib.rs 파일이 아닌 src/main.rs 파일을 가진 바이너리 크레이트라면 tests 디렉터리를 생성해 src/main.rs 파일의 함수를 use 문으로 가져와 테스트하는 통합 테스트를 작성할 수 없음
  - 오직 라이브러리 크레이트만 함수를 외부에 노출해서 다른 크레이트가 호출할 수 있음
  - 바이너리 크레이트는 코드를 직접 실행할 뿐임
- src/lib.rs 파일에 작성된 로직을 src/main.rs 파일에서 use 문을 이용해 라이브러리 크레이트의 함수를 직접 호출하여 통합 테스트를 작성해야 함

<br>

## **Summary**
- 단위 테스트는 라이브러리의 여러 부분을 개별적으로 테스트하는 것은 물론 비공개 함수도 테스트할 수 있음
- 통합 테스트는 라이브러리의 다양한 부분이 다른 코드와 잘 동작하는지 검증하며, 라이브러리의 공용 API를 외부 코드가 라이브러리를 사용하는 것과 같은 방법으로 테스트함
- 테스트는 로직의 버그를 줄이고 코드가 의도대로 동작하는지 검증하는 데 매우 중요한 역할을 함