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
- 값 캐싱 방법은 다른 코드에서 다른 클로저를 호출할 때 유용함
- Cacher 구조체의 문제
  - Cacher 인스턴스는 항상 처음으로 호출된 value 메서드의 매개변수 arg에 전달된 값과 같다는 점
    - Cacher가 하나의 값 대신 해시 맵을 저장하도록 수정해야 함
  - u32 타입의 매개변수 하나와 u32 타입의 리턴값을 가지는 클로저만 저장할 수 있다는 점
    - 문자열 슬라이스를 인수로 전달받아 usize 값을 리턴하는 클로저의 실행 결과는 캐시할 수 없음
    - 제네릭 매개변수를 사용해야 함

```rust
#[test]
fn call_with_different_value() {
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2);
    assert_eq!(v2, 2);
}
```
- 처음 c.value 메서드에 1을 전달해 호출하면 Cacher 인스턴스가 self.value 필드에 Some(1) 값을 저장함
- 이후 value 메서드에 어떤 값을 전달하든 무조건 1을 리턴함
- 해당 테스트는 실패함

<br>

#### **🤔 클로저를 이용해 주변 환경 캡처하기**

```rust
fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}
```
- 자신 주변에 선언된 변수를 참조하는 클로저
- 변수 x는 equal_to_x 클로저의 매개변수가 아닌데도 자신이 선언된 것과 같은 범위에 선언된 변수 x에 접근할 수 있음

```rust
fn main() {
    let x = 4;
    fn equal_to_x(z: i32) -> bool { z == x };
    let y = 4;
    assert!(equal_to_x(y));
}
```
- 클로저를 함수로 바꾼 코드
- **컴파일되지 않음**

<br>

- 클로저는 자신의 주변에서 값을 캡처할 때 클로저의 본문에서 사용할 값을 메모리에 저장함
  - 메모리를 이렇게 사용하면 주변의 값을 캡처할 필요가 없는 코드를 실행할 때보다는 더 많은 오버헤드가 발생함
- 함수는 주변의 환경을 캡처하지 않음
  - 주변의 값이 필요하지 않을 때는 오버헤드를 줄이기 위해 함수를 사용
- 클로저는 자신의 주변에 선언된 값을 함수가 매개변수를 저장하는 세 가지 방법과 동일하게 캡처함
  - 소유권을 가져오는 방법
    - FnOnce 트레이트는 같은 범위에 선언된 변수를 사용할 수 있음
      - 이 범위를 클로저의 '환경(environment)'이라고 함
      - 클로저는 캡처된 변수를 사용하려면 반드시 이 변수들의 소유권을 가져야 하며, 클로저를 선언하는 시점에 이 변수를 클로저 안으로 이동함
      - 트레이트의 이름에서 Once가 의미하는 것은 클로저가 같은 값에 대한 소유권을 오직 한 번만 가진다는 뜻임
  - 값을 가변으로 대여하는 방법
    - FnMut 트레이트는 값을 가변으로 대여하므로 환경에서 가져온 값을 변경할 수 있음
  - 값을 불변으로 대여하는 방법
    - Fn 트레이트는 환경에서 값을 불변으로 대여함
- 모든 클로저는 최소한 한 번은 호출될 수 있으므로 FnOnce 트레이트를 구현함
- 환경에서 가져온 값을 이동하지 않는 클로저는 FnMut 트레이트도 구현하게 되며 캡처된 변수를 변경하지 않는 클로저는 Fn 트레이트를 구현함
- 클로저가 환경에서 가져온 값에 대한 소유권을 갖게 하려면 매개변수 목록 앞에 move 키워드를 지정함
  - 이 기법은 클로저를 새로운 스레드에 전달해서 그 스레드에 데이터를 이동해서 소유하게 할 때 유용함

```rust
fn main() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    println!("변수 x를 사용할 수 없습니다: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
```
- 이 코드는 에러가 발생함
- 클로저 선언 시 변수 x는 move 키워드로 인해 클로저 안으로 이동함
- 클로저가 x의 소유권을 가지므로 main 함수는 println! 매크로를 호출할 때 x에 접근할 수 없음

<br>

### **2️⃣ 반복자를 이용해 일련의 아이템 처리하기**
- 반복자는 아이템을 순회하면서 마지막 아이템에 도달하는 때를 판단함
- 지연(lazy) 특성이 있음
  - 반복자를 변수에 저장할 때는 아이템을 순회하는 작업이 진행되지 않음
  - for 루프가 변수를 호출하면 반복자를 이용해 각 아이템을 순회하고 개별 값을 출력함
- 러스트에서는 반복자로 반복되는 로직을 줄일 수 있으며 유연성까지 가질 수 있음

#### **🤔 Iterator 트레이트와 next 메서드**
- 모든 반복자들은 표준 라이브러리에 정의된 Iterator 트레이트를 구현함

```rust
pub trait Iterator {
    type Item,
    fn next(&mut self) -> Option<Self::Item>;
}
```
- Iterator 트레이트를 구현하려면 Item 타입도 정의해야 하며, 이 Item 타입은 next 메서드의 리턴 타입으로 사용함

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```
- 반복자의 next 메서드 호출
- v1_iter 변수를 가변으로 선언함
  - next 메서드로 호출하면 이미 리턴한 값을 추적하려고 반복자 내부의 상태가 변경됨
  - 반복자를 '소비(consume)' 함
    - next 메서드 호출 때마다 반복자의 아이템을 리턴함
- v1_iter 변수를 for 루프 안에서 사용하면 루프가 v1_iter에 대한 소유권을 가지고 가변 변수로 만들기 때문에 반복자 변수를 가변 변수로 선언할 필요가 없음
- next 메서드로 얻어온 값은 벡터 안에 저장된 값에 대한 불변 참조임
  - iter 메서드는 불변 참조를 순회하는 반복자를 생성함
- v1에 대한 소유권을 가지고 소유한 값을 리턴하는 반복자 생성 시 iter 대신 into_iter 메서드 호출
  - 가변 참조 순회 시 iter 대신 iter_mut 메서드 호출

<br>

#### **🤔 반복자를 소비하는 메서드**
- 일부 메서드는 next 메서드를 호출하므로 Iterator 트레이트를 구현하려면 next 메서드를 반드시 구현해야 함
- 소비 어댑터(consuming adaptors)
  - 내부적으로 반복자를 소비하는 next 메서드를 호출하는 메서드

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}
```
- 반복자 내 아이템 총합을 얻기 위해 sum 메서드 호출
- sum 메서드는 반복자에 대한 소유권을 갖기 때문에 이 메서드를 호출한 후에는 v1_iter 변수를 더 이상 사용할 수 없음

<br>

#### **🤔 다른 반복자를 생성하는 메서드**
- 반복자 어댑터(iterator adaptors)
  - 반복자를 다른 종류의 반복자로 변경함
  - 모든 반복자는 지연 특성이 있으므로 반복자 어댑터를 호출한 후의 결과를 얻으려면 소비 어댑터 메서드 중 하나를 호출해야 함

```rust
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x+1);
}
```
- 새로운 반복자를 생성하는 반복자 어댑터 map 메서드
- 해당 코드는 아무런 작업도 수행하지 않음
  - map 메서드에 전달한 클로저는 절대 호출되지 않음

```rust
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2 = v1.iter().map(|x| x+1).collect::<Vec<_>>();
    assert_eq!(v2, vec![2, 3, 4]);
}
```
- map 메서드로 새로운 반복자 생성 후 collect 메서드로 벡터 생성
- map 메서드는 클로저를 매개변수로 사용하므로 각 아이템에 적용할 어떤 작업도 전달 가능

<br>

#### **🤔 환경을 캡처하는 클로저의 활용**
- 반복자의 filter 메서드는 반복자로부터 각 아이템을 가져와 불리언값을 리턴하는 클로저에 전달함
- 클로저가 true를 리턴하면 그 값은 filter 메서드가 생성하는 반복자에 추가되고 false를 리턴하면 추가되지 않음

```rust
#[derive(PartialEq, Debug)]

struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("스니커즈") },
        Shoe { size: 13, style: String::from("샌달") },
        Shoe { size: 10, style: String::from("부츠") },
    ];
    let in_my_size = shoe_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("스니커즈") },
            Shoe { size: 10, style: String::from("부츠") },
        ]
    );
}
```
- shoe_size 변수를 캡처한 클로저를 filter 메서드에 전달

<br>

#### **🤔 Iterator 트레이트를 이용해 직접 반복자 구현하기**
- 직접 선언한 타입에 Iterator 트레이트를 구현해서 필요한 작업 수행 가능
  - next 메서드 구현 시 Iterator 트레이트가 기본적으로 구현해 제공하는 다른 메서드도 모두 사용 가능

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```
- Counter 구조체를 선언하고 count 필드에 0을 초깃값으로 대입해 새 인스턴스를 생성하는 new 함수

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```
- Counter 구조체에 Iterator 트레이트 구현
- 반복자가 u32 값을 리턴하도록 반복자와 연관된 타입인 Item 타입은 u32로 지정
- 0으로 초기화했던 현재 상태에 1을 더해서 반복자가 1부터 리턴하도록 함
  - 만일, count 값이 6보다 작으면 next 메서드는 현재 값을 Some 값에 저장해 리턴하고, count 값이 6보다 크면 None 값을 리턴함

<br>

#### **🤔 Counter 반복자의 next 메서드 사용하기**

```rust
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```
- next 메서드 구현 테스트

<br>

#### **🤔 Iterator 트레이트의 다른 메서드 활용하기**

```rust
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a, b)| a*b)
        .filter(|x| x%3==0)
        .sum();
    assert_eq!(18, sum);
}
```
- Counter 구조체의 인스턴스가 생성한 값을 다른 Counter 인스턴스가 생성한 값 중 첫 번째 값을 제외한 나머지와 짝지어 서로를 곱한 후, 3으로 나누어 떨어지는 값만 골라 결괏값을 모두 더함
  - zip 메서드는 네 개의 쌍만 생성
    - 두 반복자 중 어느 하나가 None 값을 리턴하면 None을 리턴하므로 다섯 번째 짝인 (5, None)은 생성되지 않음
- next 메서드 동작을 구현했으므로 나머지 모든 메서드도 호출할 수 있음을 확인 가능

<br>

### **3️⃣ 입출력 프로젝트의 개선**

#### **🤔 반복자를 이용해 clone 메서드 호출 제거하기**

```rust
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err()
        })
    }
}
```
- Config::new 함수의 비효율적인 clone 메서드
  - new 함수가 String 슬라이스인 args 변수를 소유하지 않았기에 clone 메서드를 사용했음
  - 이제 new 함수가 슬라이스를 대여하는 대신 인수로 전달된 반복자의 소유권을 갖도록 수정하면 됨
    - Config::new 함수가 반복자의 소유권을 확보하고 값을 대여하는 인덱스 작업을 제거하면 clone 메서드를 사용해 새로운 메모리 할당을 수행하는 대신 반복자로부터 String 값을 Config 인스턴스로 이동할 수 있음


<br>

#### **🤔 리턴된 반복자를 직접 사용하는 방법**

```rust
use std::{env, process};
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
        process::exit(1);
    });
    println!("검색어: {}\n파일 이름: {}", config.query, config.filename);
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}
```
- env::args 함수의 리턴값을 Config::new 함수에 그대로 전달

```rust
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
```
- 반복자를 사용하도록 수정한 Config::new 함수의 시그니처
- env::args 함수는 std::env::Args 타입의 반복자를 리턴함
  - 해당 반복자를 순회해야 하므로 가변 매개변수로 선언함

<br>

#### **🤔 인덱스 대신 Iterator 트레이트의 메서드 활용하기**

```rust
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        Ok(Config {
            query: {
                match args.next() {
                    Some(arg) => arg,
                    None => return Err("검색어를 지정해야 합니다."),
                }
            },
            filename: {
                match args.next() {
                    Some(arg) => arg,
                    None => return Err("파일명을 지정해야 합니다."),
                }
            },
            case_sensitive: {
                env::var("CASE_INSENSITIVE").is_err()
            },
        })
    }
}
```
- 반복자 메서드를 사용하도록 수정한 Config::new 함수
- env::args 함수가 리턴하는 값의 첫 번째는 프로그램의 이름
  - 이 값은 무시

<br>

#### **🤔 반복자 어댑터를 이용해 더 깔끔한 코드 작성하기**

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v = vec![];
    for l in contents.lines() {
        if l.contains(query) {
            v.push(l);
        }
    }
    v
}
```
- 기존의 search 함수

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|l| l.contains(query))
        .collect()
}
```
- 반복자 어댑터를 이용해 다시 구현한 search 함수
- 함수형 프로그래밍은 가변 상태의 양을 최소화해서 코드를 더 깔끔하게 유지하는 데 도움이 됨
  - 가변 상태를 제거하면 중간값을 저장할 벡터를 사용할 필요가 없어져서 나중에 검색의 병렬 실행을 지원하기도 유리함

```rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|l| l.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
```
- 같은 방식으로 다시 구현한 search_case_insensitive 함수

<br>

#### **🤔 루프와 반복자의 성능 비교**
- 반복자가 고수준의 추상화를 제공하기는 하지만 직접 작성하는 저수준의 코드와 거의 같은 코드로 컴파일됨
- 반복자는 러스트의 무비용 추상화(zero-cost abstractions) 기능 중 하나임
  - 추상화를 사용한다고 추가적인 런타임 오버헤드가 발생하지 않음

```rust
fn main() {
    let buf: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;

    for i in 12..buf.len() {
        let prediction = coefficients.iter()
            .zip(&buf[i-12..i])
            .map(|(&c, &s)| c*s as i64)
            .sum::<i64>() >> qlp_shift;
        let delta = buf[i];
        buf[i] = prediction as i32 + delta;
    }
}
```
- prediction 값을 계산하기 위해 coefficients 배열의 12개 값을 순회하면서 zip 메서드를 이용해 buf에 저장된 이전 12개 값을 이용해 값의 쌍을 만듬
  - 각 쌍의 값들을 곱하고 그 결과를 모두 더해서 qlp_shift 변수에 지정된 비트만큼 오른쪽으로 이동함
- 해당 코드는 개발자가 손으로 작성하는 것과 같은 어셈블리 코드로 컴파일됨
  - coefficients 배열의 값을 순회하는 데 필요한 루프는 존재하지 않지만 러스트는 12개의 아이템을 순회해야 한다는 것을 알고 있으므로 루프를 풀어냄
  - 여기서 풀어낸다는 것은 루프를 제어하는 코드의 오버헤드를 없애기 위해 루프를 제거하고 루프 안에서 실행되던 코드를 필요한 횟수만큼 반복하는 코드를 생성하는 과정을 뜻함
- coefficients 배열의 모든 값들은 레지스터에 저장됨
  - 값에 매우 빠르게 접근 가능
  - 배열 접근 시 런타임에 경계값 검사도 실행하지 않음

<br>

## **Summary**
- 클로저와 반복자는 함수형 프로그래밍 언어로부터 차용한 기능
- 클로저와 반복자의 구현은 런타임 성능에 거의 영향을 미치지 않음
  - 무비용 추상화를 달성하기 위한 노력의 일환