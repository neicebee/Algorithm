# 💻 Rust String
- Rust의 String은 컬렉션에 포함됨
  - String은 바이트의 컬렉션 및 이 바이트들을 텍스트로 표현할 때 유용한 기능을 제공하는 메서드들로 구현되어 있기 때문
- Rust의 String 타입
  - `String`
  - `str`

## 🤔 `String` 개념
- `std` lib 에 의해 제공되는 `String` 타입
- `String` 은 확장 가능한 8비트 부호 없는 배열인 `Vec<char>` 를 포함하는 구조체
- growable, mutable, ownable, Encode with UTF-8, **_Not null terminated_**
- `String` 은 `str` 과 달리 데이터 소유권 보유
  - 때문에 `String` 값을 변수에 할당할 때 `&` 또는 차용 상태를 활용하는 것이 필수적이지 않음
- 초기화하는 동안 `String` 의 크기는 컴파일 타임에 알려지거나 알려지지 않을 수 있지만 길이가 한계에 도달할 때까지 확장 가능

## 🤔 `str` 개념
- `str` 은 러스트 핵심 언어 기능 내에 존재하는 유일한 String 타입: `&[char]` 슬라이스
  - 문자열 리터럴을 정의하는 기본 유형
  - `&str` 은 참조자 형태
    - 다른 어딘가에 저장된 UTF-8로 인코딩된 스트링 데이터의 참조자
    - 프로그램 바이너리에 위치하는 스트링 리터럴도 스트링 슬라이스라고 할 수 있음

### 💀 String slice
- 항목 시퀀스를 포함하고 구문으로 표시되는 보기
- 소유권이 없지만 항목이 나타나는 순서 참조 가능
- 결과적으로, 스트링 슬라이스는 문자열 요소 시퀀스에 대한 참조
- 확장 가능한 배열인 `String` 에는 각 문자에 대한 위치 또는 인덱스가 포함되어 있음

### 💀 String literal
- 텍스트를 큰 따옴표로 묶어 구성
  - 읽기 전용 메모리에 실행 파일의 일부로 저장된 `"사전 할당된 텍스트"` 를 참조하는 문자열 조각
    - 즉, RAM은 소프트웨어와 함께 제공되며 스택 캐시에 의존하지 않음

## 🤔 `String` 과 `str` 차이점
| String | str |
| ------ | ------|
| mutable | immutable |
| 컴파일 시 크기 알 수 없음 | 컴파일 시 크기 알 수 있음 |
| 데이터 힙에 저장 | 데이터 프로그램 바이너리 메모리 위치에 저장 |
| 변수 값 할당하기 위해 사용하거나 참조할 필요 없음 | 단일 변수 할당 시 사용하거나 참조 |

## 🤔 Code
```rust
// 변수의 타입을 출력하는 함수
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let test_string = String::from("test!!!");
    print_type_of(&test_string); // alloc::string::String
    println!("{}", test_string); // test!!!

    let test_str = "test!!!";
    print_type_of(&test_str); // &str
    println!("{}", test_str); // test!!!

    // 문자열 슬라이싱
    let test_slice = &test_string[1..4];
    print_type_of(&test_slice); // &str
    println!("{}", test_slice); // est
}
```