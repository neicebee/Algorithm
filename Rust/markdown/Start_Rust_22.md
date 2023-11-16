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