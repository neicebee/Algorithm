# 🦀 Rust Day 3

## 숫자 맞추기 게임 구현 파트의 `반복문`

### Code

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The number of player must guess: {}", secret_number);
    println!("The number of player thinks.");

    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Unable to read input!");
        let guess: u32 = guess.trim().parse()
            .expect("Input Error!");
        
        println!("Input Number: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Your number is Less."),
            Ordering::Greater => println!("Your number is Greater."),
            Ordering::Equal => {
                println!("Correct!!");
                break;
            }
        }
    }
}
```

- `loop`: 무한 반복 실행
  - 사용자의 입력값을 묻는 코드를 loop 안으로 옮김
  - 하지만 끝없이 사용자의 입력값을 요구하게 됨
  - 무한 루프에서 탈출할 방법
    - `ctrl+c`: 프로그램 강제 종료
    - 숫자가 아닌 값 입력: 에러로 인한 프로그램 종료
  - 숫자를 맞추면 종료하기 위해 `Ordering::Equal =>` 코드에 `break` 구문 추가

## 숫자 맞추기 게임 구현 파트의 `오입력 처리`

### Code

```rust
use core::num;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The number of player must guess: {}", secret_number);
    println!("The number of player thinks.");

    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Unable to read input!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Input Number: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Your number is Less."),
            Ordering::Greater => println!("Your number is Greater."),
            Ordering::Equal => {
                println!("Correct!!");
                break;
            }
        }
    }
}
```

- 사용자가 올바르지 않은 숫자를 입력했을 때 프로그램을 그냥 종료하는 대신 매끄럽게 처리하기 위함
  - 숫자가 아닌 값 입력 -> 무시 -> 재입력
- guess 변수를 형 변환하는 코드 수정
  - ```rust 
        let guess: u32 = guess.trim().parse()
            .expect("Input Error!!");
        => let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    ```
    - `expect` 메서드 -> `match` 표현식
- `parse` 메서드
  - 문자열을 숫자로 문제없이 변환하면 변환된 숫자를 가진 Ok 값 리턴
  - 숫자로 변환하지 못하면 관련된 에러 정보를 담은 Err 값 리턴
- `_`: 모든 값을 표현하는 문자

## Final Code

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("The number of player thinks.");
        
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Unable to read input!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Input Number: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Your number is Less."),
            Ordering::Greater => println!("Your number is Greater."),
            Ordering::Equal => {
                println!("Correct!!");
                break;
            }
        }
    }
}
```