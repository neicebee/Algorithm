# 🦀 Rust Day 3

## 숫자 맞추기 게임 구현 파트의 `난수 생성`

### rand crate

- 난수를 생성하기 위해서는 `rand`라는 crate가 필요함

- `crate`: 소스 파일의 집합
  - 러스트의 기본 패키지, 혹은 라이브러리 단위
  - 우리가 작업 중인 프로젝트도 실행이 가능한 `binary crate`
  - 파이썬의 Library 개념이라고 생각하면 이해하기 편할 듯

- `Cargo.toml` 파일을 수정하여 의존 패키지로 등록해야 함
  - 의존 패키지 추가 시 Cargo는 패키지가 등록된 repository인 `https://crates.io`에서 최신 복사본을 내려받음
    - `https://crates.io`: 러스트 개발자들의 오픈 소스 프로젝트 저장소

### Cargo.lock

- 최초 cargo build 명령 실행 시 생성
- 러스트는 lock 파일의 crate 버전을 베이스로 실행함
- Cargo의 update 명령은 lock 파일에 명시된 버전을 무시하고 toml 파일에 지정된 조건에 해당하는 가장 최신 버전을 다시 찾음. 성공적으로 실행 시 lock 파일의 버전을 갱신함

### Code

```rust
use std::io;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The number of player must guess: {}", secret_number);
    println!("The number of player thinks.");
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Unable to read input!");

    println!("Input Number: {}", guess);
}
```

- `use rand::Rng`: Rng trait는 난수 생성기에 구현된 메서드를 정의함
  - `trait`: 불특정 타입을 대상으로 이들이 구현해야 할 메서드의 집합을 정의
- `rand::thread_rng()`: 난수 생성기 반환
  - 현 코드를 실행 중인 스레드 내에 존재하며 OS가 지정한 seed 값을 사용
- `gen_range(1, 101)`: Rng trait에 정의되어 있음. 인수로 지정된 두 값 사이의 난수를 선택해 반환함

## 숫자 맞추기 게임 구현 파트의 `비교`

### Code

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    // println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The number of player must guess: {}", secret_number);
    // println!("The number of player thinks.");
    
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess)
    //     .expect("Unable to read input!");

    // println!("Input Number: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Your number is Less."),
        Ordering::Greater => println!("Your number is Greater."),
        Ordering::Equal => println!("Correct!!"),
    }
}
```

- `std::cmp::Ordering`: Order 열거자. Less, Greater, Equal 중 하나 표현
  - cmp 메서드는 두 개의 값을 비교함. 비교 가능한 어떤 값에도 적용할 수 있음
    - 비교 결과를 Ordering 열거자의 열것값 중 하나 반환
- `match` 표현식
  - 여러 개의 가지로 구성됨. 각각의 가지는 패턴, 그리고 match 표현식의 시작 부분에 주어진 값이 패털과 일치할 때 실행될 코드로 구성됨
  - match 표현식에 지정된 값을 읽어 각 가지의 패턴과 차례대로 비교
  - C 언어의 switch case 문과 비슷하다는 생각이 듬
- 실행 시 `타입 불일치` 에러가 발생함
  - 러스트는 정적인 타입 시스템을 갖췄지만 타입 추론(type inference)도 지원함
  - 때문에 변수 `guess`가 String 타입이어야 한다는 것을 스스로 이해함
  - 변수 `secret_number`는 숫자 타입
    
    1. 32bit 정수 `i32`: signed 32-bit number
    2. 32bit 부호 없는 정수 `u32`: unsigned 32-bit number
    3. 64bit 정수 `i64`: signed 64-bit number

  - 러스트는 숫자 값에 대해서 기본적으로 `i32` 타입을 사용하며 문자열과 숫자 타입 비교가 불가능함 => **_문자열로 읽은 값을 실제 숫자로 변환해야 비교 가능_**
  - 변환 코드 추가
    ```rust
    let guess: u32 = guess.trim().parse().expect("Input Error!");
    ```
    - 변수 가리기(shadowing): 어떤 타입의 값을 다른 타입으로 변환할 때 주로 사용. 때문에 두 개의 변수를 선언할 필요가 없음
    - `trim()`: String 타입에 정의되어 있음. 문자열 양쪽 끝의 공백을 제거함
    - `parse()`: 문자열에 정의되어 있음. 문자열을 구문 분석(parsing)해서 숫자로 변환함. 다양한 타입의 숫자 구문 분석이 가능하기 때문에 원하는 숫자 타입을 정확히 명시해야 함
      - Result 타입의 열것값(Ok, Err) 중 하나를 반환
      - expect() 사용

**_u32: 작은 범위의 양수를 처리하기에 알맞은 숫자 타입_**