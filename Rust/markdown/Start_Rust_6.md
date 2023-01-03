# 🦀 Rust Day 6

## **🏳️ Common Programming Concepts**

### **3️⃣ 함수 (functions)**
- 프로그램의 진입점 : `main function`
- 새로운 `function` 선언 : `fn` keyword
- 러스트는 함수와 변수 이름에 `snake case` 사용
- 러스트에서 함수의 선언 순서는 중요하지 않으며 어디에 선언했는지가 중요

#### **🤔 함수 매개변수**
- 매개변수(parameter) : 함수의 signature에 포함되는 특별한 변수
  - 이 매개변수에 _구체적인 값_ 을 전달할 수 있음
  - _구체적인 값_ : 인수(argument)
- 함수 signature에는 각 매개변수 타입을 명시해야 함

#### **🤔 함수 본문의 구문과 표현식**
- 함수 본문은 여러 개의 구문(statements)으로 구성되며, 선택적으로 표현식(expression)으로 끝나기도 함
- 러스트는 표현식 언어 기반
  - 구문과 표현식을 구분하는 것은 매우 중요함
- **☝️ 구문**
  - 어떤 동작을 실행하지만 **_값을 리턴하지 않는_** 명령
    - `let` keyword 이용해 변수 생성 후 값 대입
    - 함수 선언
  - 즉, `let` 구문은 다른 변수에 대입 불가능
  - **_대입문이 대입된 값을 리턴하는 C나 루비 같은 언어와 다른 점_**
    - C나 루비 같은 언어에서는 `x = y = 6` 과 같은 구문을 작성하면 x와 y 변수의 값이 모두 6임
    - 하지만 러스트에서 구문은 값을 리턴하지 않으므로 같은 결과를 얻을 수 없음
- **☝️ 표현식**
  - 최종 결괏값으로 평가(evaluate)됨
    - `5+6` 사칙연산은 `11` 이라는 값이 평가됨
    - 함수의 호출
    - 매크로의 호출
    - 새로운 범위 선언 시 사용하는 코드 블록
  - 즉, 표현식은 구문의 일부가 될 수 있음
  - **_표현식은 마지막에 세미콜론을 포함하지 않음_**
    ```rust
    fn main() {
        let x = 5;
        let y = {
            let x = 3;
            x + 1
        };
        println!("y의 값: {}", y);
    }
    // Result 
    // y의 값: 4
    ```
    - 만약 세미콜론을 추가하면 표현식이 구문으로 바뀌어 값을 리턴하지 않음

#### **🤔 값을 리턴하는 함수**
- 리턴할 값의 타입을 `->` 로 지정해야 함
- 러스트에서 함수의 리턴값은 함수 본문의 마지막 표현식 값
- `return` keyword로 반환값 지정이 가능하지만 대부분의 함수는 마지막 표현식의 결과를 리턴함

### **4️⃣ 주석**
- 두 개의 슬래시 사용 => `//`
- 코드의 끝에 작성해도 됨
- 하지만 대부분 주석은 관련된 코드의 윗줄에 작성하는 것이 일반적
- 러스트는 문서화 주석(document comments)이라는 특별한 형식의 주석도 지원

### **5️⃣ 흐름 제어**
- 프로그램의 실행 흐름을 제어할 수 있는 가장 일반적인 구조
  - if 표현식
  - loop

#### **🤔 if 표현식**
- `if` keyword 다음 조건식 부여
- if 문의 조건은 반드시 `boolean` 타입 중 하나를 리턴해야 함
- 루비나 JS 같은 언어와 달리 러스트는 `boolean` 이 아닌 값을 `boolean` 으로 자동 변환 불가능
- `else if` 표현식을 너무 많이 사용하면 코드가 지저분해보임
  - 코드 리팩토링(refactoring) 필요 => `match` 표현식 사용
- if 문은 표현식이므로 `let` 구문에 적용 가능
    ```rust
    fn main() {
        let condition = true;
        let num = if condition{
            1
        } else {
            0
        };
        println!("num의 값: {}", num);
    }
    // Result
    // num의 값: 1
    ```
- if 구문의 각 가지가 리턴하는 결과가 모두 같은 타입이어야 함
    ```rust
    fn main() {
        let condition = true;
        let num = if condition{
            1
        } else {
            "zero"
        };
        println!("num의 값: {}", num);
    }
    // Result
    // error[E0308]: `if` and `else` have incompatible types
    ```
- **Why?**
  - **_코드 블록의 결과는 마지막 표현식의 값을 평가하며 숫자 자체도 하나의 표현식. if 표현식 전체의 결과는 어떤 코드 블록이 실행되는가에 따라 달라지기 때문에 if 구문의 각 가지가 리턴하는 결과는 모두 같은 타입이어야 함_**
  - 러스트는 변수를 사용하는 모든 코드의 유효성을 검사하기 위해 컴파일 시점에 변수의 타입이 무엇인지 알아야 함. 변수의 타입이 런타임에 결정된다면 러스트는 이런 유효성 검사 수행 불가능. 그러면 컴파일러의 구조가 더 복잡해질뿐더러 변수의 타입 변경을 추적해야 한다면 컴파일러는 지금만큼 코드의 실행을 보장할 수 없음

#### **🤔 loop**
- `loop`, `while`, `for` 등 세 가지 종류의 루프 제공
- **☝️ `loop` keyword**
  - 루프를 중지하라고 명시하지 않는 한 무한 반복
  - `break` keyword 사용 가능
- **☝️ `loop` 에서 값 리턴하기**
  - 스레드가 작업을 완료했는지 여부를 확인하는 등 실패할 가능성이 있는 작업을 재시도하는 경우 `loop` 사용
  - 작업의 결과를 다른 코드에 전달해야 할 수도 있음
  - `break` keyword 다음에 리턴값 추가
- **☝️ `while`** 
  - 조건식이 일치하는 동안에는 코드가 실행되며, 그렇지 않으면 루프 탈출
- **☝️ `for` 을 이용해 컬렉션을 반복 처리**
  - iterator 사용
    ```rust
    fn main() {
        let a = [1, 2, 3, 4, 5];
        for i in a.iter(){
            println!("Element value: {}", i);
        }
    }
    // Result
    // Element value: 1
    // Element value: 2
    // Element value: 3
    // Element value: 4
    // Element value: 5
    ```
  - 표준 lib의 Range 타입 이용
    ```rust
    fn main() {
        for i in 1..6{
            println!("Element value: {}", i);
        }
    }
    // Result
    // Element value: 1
    // Element value: 2
    // Element value: 3
    // Element value: 4
    // Element value: 5
    ```
  - 범위를 뒤집어서 생성하는 `rev` 메서드와 같이 응용
    ```rust
    fn main() {
        for i in (1..4).rev(){
            println!("{}...", i);
        }
        println!("발사!");
    }
    // Result
    // 3...
    // 2...
    // 1...
    // 발사!
    ```