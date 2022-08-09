# 🦀 Rust Day 2

## 숫자 맞추기 게임 구현 파트의 변수와 입출력

### Code

```rust
use std::io;

fn main() {
    println!("랜덤 숫자를 맞혀보세요!");
    println!("예상 숫자를 입력하세요.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("입력한 값을 읽지 못했습니다.");

    println!("입력한 값: {}", guess);
}
```

- `use std::io`
  - 입출력 관련 라이브러리
  - std 표준 라이브러리의 io 라이브러리를 사용하겠다는 의미
- `let mut`
  - let: 변수 생성
  - mut: mutable, 값을 변경할 수 있는 변수임을 정의
    - _Rust의 변수는 기본적으로 값을 변경할 수 없음(immutable)_
- `guess = String::new()`
  - String::new()의 실행 결과를 변수 guess의 바인딩 값으로 지정
  - `::` => new()가 String 타입의 연관 함수(associated function)라는 것을 의미
    - new(): 빈 문자열 생성
    - 연관 함수(associated function): 특정한 인스턴스가 아니라 타입 자체에 구현된 함수 == Static method
- `io::stdin().read_line()`
  - [stdin, stdout, stderr에 대한 참조](https://it-neicebee.tistory.com/118) 
  - `use std::io`를 사용하지 않는다면 `std::io::stdin()`으로 사용가능
- &: 참조(reference) 타입
- {}: 자리 지정자(place holder)
- `read_line(String)`: io::Result 타입의 값 리턴
  - Result - enums(Ok, Err)
    - `expect()`: Result 타입의 연관 함수
    - Result 타입의 인스턴스가 Err 값이라면 expect 메서드는 프로그램의 실행을 종료하고 expect 메서드에 인수로 전달한 메시지를 표시. Ok라면 expect 메서드는 Ok 값이 보관하고 있는 값만을 읽어와 리턴
      - _Python의 예외처리 try, except 문과 비슷함_