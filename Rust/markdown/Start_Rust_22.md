# 🦀 Rust Day 22

## **🏳️ Functional Language Features: Iterators and Closures**
- Closures : 변수에 저장할 수 있는 함수 형식의 구조
- Iterators : 일련의 원소들을 처리하는 방법

<br>

### **1️⃣ 클로저: 주변 환경을 캡처하는 익명 함수**
- 변수에 저장하거나 다른 함수에 인수로 전달하는 익명 함수(anonymous functions)
- 일반 함수와 달리 클로저는 자신이 정의된 범위 내의 값들을 '캡처(capture)'함

#### **🤔 클로저를 이용한 동작의 추상화**

```rust
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("시간이 오래 걸리는 계산을 수행 중...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
```
- 실행에 2초가 걸리는 가상의 계산을 수행하는 simulated_expensive_calculation 함수

```rust
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}
```
- 사용자의 입력과 임의의 숫자를 하드코딩한 값을 이용하는 main 함수

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "오늘은 {}번의 팔굽혀펴기를 하세요!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "다음에는 {}번의 윗몸 일으키기를 하세요!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("오늘은 수분을 충분히 섭취하며 쉬세요!");
        } else {
            println!(
                "오늘은 {}분간 달리기를 하세요!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```
- 입력값에 따라 simulated_expensive_calculation 함수를 호출해서 운동 계획을 생성하는 로직
- simulated_expensive_calculation 함수를 단 한 번만 호출하도록 리팩토링 필요

<br>

**(1)** 함수를 위한 리팩토링

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!(
            "오늘은 {}번의 팔굽혀펴기를 하세요!",
            expensive_result
        );
        println!(
            "다음에는 {}번의 윗몸 일으키기를 하세요!",
            expensive_result
        );
    } else {
        if random_number == 3 {
            println!("오늘은 수분을 충분히 섭취하며 쉬세요!");
        } else {
            println!(
                "오늘은 {}분간 달리기를 하세요!",
                expensive_result
            );
        }
    }
}
```
- simulated_expensive_calculation 함수 호출을 한 번만 수행하고 그 결과를 expensive_result 변수에 저장

**(2)** 코드를 저장하는 클로저를 이용한 리팩토링

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("시간이 오래 걸리는 계산을 수행 중...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "오늘은 {}번의 팔굽혀펴기를 하세요!",
            expensive_closure(intensity)
        );
        println!(
            "다음에는 {}번의 윗몸 일으키기를 하세요!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("오늘은 수분을 충분히 섭취하며 쉬세요!");
        } else {
            println!(
                "오늘은 {}분간 달리기를 하세요!",
                expensive_closure(intensity)
            );
        }
    }
}
```
- 클로저를 선언하고 expensive_closure 변수에 저장
  - 클로저 선언은 변수에 값을 할당하는 = 다음부터 시작함
  - 클로저 선언 시 파이프 문자의 쌍이 필요함
    - 파이프 문자 사이에는 클로저의 매개변수를 지정할 수 있음
  - 클로저를 사용하는 이유는 코드를 한 곳에 정의하고 그 코드를 저장해서 필요할 때 나중에 호출하기 위함
- expensive_closure 클로저를 호출하는 generate_workout 함수

<br>

#### **🤔 클로저의 타입 추론과 애노테이션**

```rust
let expensive_result = |num: u32| -> u32 {
    println!("시간이 오래 걸리는 계산을 수행 중...");
    thread::sleep(Duration::from_secs(2));
    num
}
```
- 클로저 선언 시 함수처럼 매개변수와 리턴값의 타입을 지정할 필요가 없음
  - 클로저는 변수에 저장되고 이름도 없으며 라이브러리의 사용자에게 노출되지도 않음
- 변수와 마찬가지로 가독성을 높이는 일이 장황한 코드를 작성하는 것보다 더 중요할 때 클로저에도 타입 애노테이션을 추가할 수 있음

```rust
fn add_one_1(x: u32) -> u32 { x+1 } // 함수의 정의
let add_one_2 = |x: u32| -> u32 { x+1 }; // 타입 애노테이션을 적용한 클로저 선언
let add_one_3 = |x| { x+1 }; // 클로저 선언에서 타입 애노테이션 제거
let add_one_4 = |x| x+1; // 하나의 표현식으로만 구현된 클로저 선언이므로 괄호까지 제거함
```

```rust
let example_closure = |x| x
let s = example_closure(String::from("hello"));
let n = example_closure(5);
```
- 처음 String 타입을 인수로 넣어 호출한 클로저는 다른 타입을 인수로 넣어 호출하면 에러가 발생함
  - 처음 타입 정보가 해당 클로저에 기록되기 때문

<br>

#### **🤔 제네릭 매개변수와 Fn 트레이트를 이용해 클로저 저장하기**
- `메모이제이션(memoization) or 지연 평가(lazy evaluation)` 기법
  - 클로저와 클로저의 실행 결과를 저장할 구조체 선언
  - 구조체는 결괏값이 필요할 때만 클로저를 실행한 후 그 결괏값을 캐싱함
  - 나머지 코드에서는 그 결괏값을 따로 저장하고 재사용하는 코드를 작성하지 않아도 됨

```rust
struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}
```
- 클로저와 리턴값을 calculation과 value 필드에 저장하는 Cacher 구조체
  - 구조체 선언 시 각 필드의 타입을 알아야 하므로 클로저의 타입을 명시해야 함
  - 클로저 인스턴스는 각자 유일한 익명 타입을 가지고 있음
    - 두 개의 클로저가 같은 시그니처를 갖더라도 서로 다른 타입으로 인식함
  - 제네릭 T 타입의 calculation 필드
    - T 타입은 Fn 트레이트 경계를 지정해 이 타입이 클로저여야 하나는 사실을 명시함
  - Option<u32> 타입의 value 필드
    - 클로저를 실행하기 전에는 None 값임
    - 구조체를 이용하는 코드가 클로저의 결과를 요청하면 그 시점에 클로저를 호출한 뒤 결괏값을 Some 열것값에 저장해 필드에 저장

```rust
impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```
- Cacher 구조체의 캐싱 로직
  - 다른 코드가 구조체의 value 필드값을 임의로 변경하기를 원치 않기 때문에 이 필드는 비공개로 선언함

```rust
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(
        |num| {
            println!("시간이 오래 걸리는 계산을 수행 중...");
            thread::sleep(Duration::from_secs(2));
            num
        }
    );

    if intensity < 25 {
        println!(
            "오늘은 {}번의 팔굽혀펴기를 하세요!",
            expensive_result.value(intensity)
        );
        println!(
            "다음에는 {}번의 윗몸 일으키기를 하세요!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("오늘은 수분을 충분히 섭취하며 쉬세요!");
        } else {
            println!(
                "오늘은 {}분간 달리기를 하세요!",
                expensive_result.value(intensity)
            );
        }
    }
}
```
- generate_workout 함수에서 Cacher 구조체를 이용해 캐싱 로직 추상화

<br>

#### **🤔 Cacher 구현의 한계**
