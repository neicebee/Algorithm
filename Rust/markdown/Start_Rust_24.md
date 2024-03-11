# 🦀 Rust Day 24

## **🏳️ Smart Pointers**
- 포인터(pointer) : 메모리에 주소를 가지고 있는 변수를 일컫는 보편적인 개념
- 러스트의 참조
  - & 기호 이용해 지정
  - 변수가 가리키는 값을 대여함
  - 별도의 오버헤드 없음
- 스마트 포인터(smart pointer) : 포인터처럼 동작할 뿐만 아니라 추가적인 메타데이터와 기능을 포함하는 데이터 구조
  - 참조는 데이터를 대여할 수만 있지만, 스마트 포인터는 대부분 포인터가 가리키는 데이터를 소유함
  - `String` 과 `Vec<T>` 타입 모두 메모리를 소유하며 데이터를 갱신할 수 있으므로 스마트 포인터
    - 메타데이터(저장 용량 등)와 추가 기능이나 보장(String은 모든 데이터가 항상 유효한 UTF-8 문자임을 보장)을 제공
  - 스마트 포인터는 주로 구조체를 이용해 구현함
    - 일반 구조체와 달리 Deref와 Drop 트레이트를 구현함
    - Deref : 스마트 포인터 구조체가 참조처럼 동작해서 참조나 스마트 포인터를 같은 방법으로 다룰 수 있도록 지원
    - Drop : 스마트 포인터의 인스턴스가 범위를 벗어날 때 임의의 코드를 실행하도록 지원

### **1️⃣ `Box<T>` 를 이용해 힙 메모리의 데이터 참조하기**
- 박스(`Box<T>`) : 가장 직관적인 스마트 포인터 
  - 데이터를 힙 메모리에 저장함
  - 스택에는 힙 데이터를 가리키는 포인터만 저장함
  - 박스를 사용하는 예
    - 컴파일타임에 크기를 알 수 없는 타입을 정확한 크기가 필요한 상황에 사용하려고 할 때
    - 데이터의 크기가 커서 데이터를 복제하지 않고 소유권을 이전하고 싶을 때
    - 특정 타입이 아니라 특정 트레이트를 구현하는 타입의 값을 소유하고자 할 때

#### **🤔 `Box<T>` 를 이용해 힙 메모리에 데이터 저장하기**

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```
- 박스를 이용해 i32값을 힙 메모리에 저장
  - 소유된 값과 마찬가지로 박스가 범위를 벗어나면 메모리가 해제됨(박스와 박스가 가리키는 데이터의 메모리가 모두 해제)

<br>

#### **🤔 박스를 이용해 재귀 타입 활용하기**
- 재귀 타입(recursive type) : 같은 타입의 다른 값을 자신의 일부에 포함하는 값
  - 값을 중첩하는 것은 이론적으로 무한할 수 있어서 러스트는 재귀 타입의 값을 저장하는 데 필요한 공간을 판단할 수 없음
  - 박스의 크기는 정해져 있으므로 재귀 타입을 선언할 때 박스를 활용함

<br>

**(1)** 리스트 생성자에 대한 더 자세한 정보
- 콘스 리스트(cons list; construction list) : 리스프(Lisp) 프로그래밍 언어 및 파생 언어에 도입된 데이터 구조
  - 리스프의 cons 함수는 보통 하나의 값과 값의 쌍으로 구성된 두 개의 인수를 이용해 새로운 쌍을 생성하며 이 쌍을 포함하는 쌍이 리스트를 구성하게 됨
  - 콘스 리스트의 각 아이템은 현재 아이템의 값과 그 다음 아이템의 값 등 두 개의 원소로 구성됨
    - 리스트의 마지막 아이템은 다음 아이템이 없으므로 Nil 이라는 값을 가짐
  - 이 리스트는 콘스 함수를 재귀적으로 호출해서 생성함
  - 재귀의 기본 상태(base case)를 뜻하는 정식 명칭은 Nil
  - 콘스 리스트는 함수형 프로그래밍 언어에서는 사용 빈도가 높지만, 러스트는 보편적으로 사용하지 않음
- 'x를 y에 콘스한다' : 원소 x를 y 리스트의 첫 부분에 추가해서 새로운 리스트를 생성한다는 의미

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```
- i32 값의 콘스 리스트 데이터 구조를 표현하는 열거자

```rust
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```
- List 열거자를 이용해 1, 2, 3 목록 저장
- `recursive type List has infinite size` 에러 발생
  - 어느 정도의 메모리 공간이 필요한지 판단할 수 없기 때문

<br>

**(2)** 비재귀 타입의 크기를 계산하는 방법

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
- 러스트는 Message 값을 저장하는 데 필요한 공간을 결정하기 위해 각 열것값을 모두 확인해서 어떤 열것값이 더 많은 공간을 차지하는지 확인함
- 단 하나의 열것값만 사용하므로 Message 값이 필요로 하는 공간은 가장 큰 열것값을 저장하는 데 필요한 공간임

```rust
enum List {
    Cons(i32, List),
    Nil,
}
```
- 컴파일러는 Cons 열것값을 먼저 확인함
  - i32 타입과 List 타입의 값 저장
  - Cons는 i32 타입의 크기에 List 타입의 크기를 더한 공간이 필요함
  - List 타입에 필요한 공간을 확인하기 위해 컴파일러는 다시 Cons 열것값부터 살펴보기 시작함 -> 무한하게 반복됨

<br>

**(3)** `Box<T>` 를 이용해서 재귀 타입의 크기 결정하기

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2, 
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```
- 크기를 알고 있는 박스를 이용해 다시 선언한 List 열거자
  - `Box<T>` 는 포인터이므로 러스트는 박스가 필요한 공간이 얼마인지 알고 있음
  - `Box<T>` 는 Cons 열것값에 저장된 데이터가 아니라, 힙 메모리에 저장된 다음 List의 값을 가리킴
    - 이론적으로는 다른 리스트를 '저장하고 있는' 리스트를 가지게 됨
    - 한 리스트 안에 다른 리스트를 저장하는 것이 아닌 한 리스트 옆에 다른 아이템을 저장하는 셈
- 박스는 값의 간접적인 저장과 힙 메모리 할당만 지원하므로 어떠한 성능 오버헤드도 발생하지 않음

<br>

### **2️⃣ Deref 트레이트를 이용해 스마트 포인터를 참조처럼 취급하기**
- Deref 트레이트 구현 시 역참조 연산자 `*` 의 동작을 변경할 수 있음
  - 스마트 포인터를 참조처럼 취급할 수 있어 참조를 사용하는 코드를 그대로 스마트 포인터에도 적용 가능

#### **🤔 역참조 연산자를 이용해 포인터가 가리키는 값 읽어오기**
- 참조 : 일정의 포인터로 어딘가에 저장된 값을 가리키는 이정표

```rust
fn main() {
    let x = 5;
    let y = &x;
    
    assert_eq!(5 ,x);
    assert_eq!(5, *y);
    // assert_eq!(5, y) => 에러 발생
    // 숫자와 숫자에 대한 참조는 서로 다른 타입
}
```
- 역참조 연산자를 이용해 i32 값의 참조 따라가기
  - 변수 x는 i32 값인 5 저장
  - 변수 y는 x에 대한 참조 저장
    - y의 값 검증 시 *y 처럼 역참조 연산자를 덧붙여 변수가 가리키는 값의 참조를 따라가야 함
      - 즉, y를 역참조 시 y가 가리키는 값 5에 접근 가능

<br>

#### **🤔 `Box<T>` 를 참조처럼 사용하기**

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);
    
    assert_eq!(5 ,x);
    assert_eq!(5, *y);
}
```
- 변수 y에 x의 값을 가리키는 참조 대신 x의 값을 가리키는 박스의 인스턴스 대입

<br>

#### **🤔 직접 구현하는 스마트 포인터**

```rust
struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```
- T 타입의 원소 하나를 갖는 튜플 구조체 MyBox
- new 함수는 매개변수 하나를 가지며 그 값을 저장하는 MyBox 인스턴스 리턴

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
// Result
// type `MyBox<{integer}>` cannot be dereferenced
```
- `MyBox<T>` 타입은 역참조를 지원하는 Deref 트레이트를 구현하지 않았으므로 * 연산자의 역참조를 지원하지 않음

<br>

#### **🤔 Deref 트레이트를 구현해서 타입을 참조처럼 이용하기**

```rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T; // Deref 트레이트가 사용할 연관 타입 선언

    fn deref(&self) -> &T {
		// deref 메서드가 * 연산자를 통해 참조하고자 하는 값을 리턴
        &self.0
    }
}
```
- `MyBox<T>` 타입에 Deref 트레이트 구현
- Deref 트레이트를 구현하지 않으면 컴파일러는 & 참조만 역참조 가능

```rust
assert_eq!(5, *y);
*(y.deref()); // *y 코드 도달 시 내부적 실행 코드
```
- 러스트는 `*` 연산자를 deref 메서드 호출로 변환하고 역참조를 해석해서 개발자가 deref 메서드를 호출해야 하는지를 직접 결정하지 않도록 지원함
- deref 메서드가 값에 대한 참조를 리턴하는 이유
  - 소유권 시스템이 `*(y.deref())` 구문에서 괄호 바깥의 역참조를 요구하기 때문
  - deref 메서드가 값의 참조가 아니라 값을 직접 리턴하면 리턴된 값이 self 참조로 이동함
- 코드에서 `*` 연산자를 사용할 때마다 `*` 연산자는 deref 메서드 호출로 교체된 후 `*` 연산자를 한 번만 호출함
  - `*` 연산자 교체는 무한한 재귀호출이 아니므로 값을 얻게 됨

<br>

#### **🤔 함수와 메서드에 묵시적인 Deref 강제하기**
- 역참조 강제(Deref coercion) : Deref 트레이트를 구현하는 타입의 참조를 Deref 트레이트가 변환하는 원래의 타입에 대한 참조로 변환함
  - 특정 타입 값의 참조를 함수나 메서드의 인수로 전달할 때나 인수가 함수나 메서드의 매개변수 타입과 일치하지 않을 때 자동으로 처리됨
  - 때문에 함수나 메서드 호출 시 &와 * 연산자를 이용해 명시적으로 참조나 역참조를 수행할 필요가 없음

```rust
fn hello(name: &str) {
    println!("Hello, {name}!");
}
```
- &str 타입의 매개변수 name을 사용하는 hello 함수

```rust
fn main() {
    let msg = MyBox::new("World");
    hello(&msg);
}
```
- 역참조 강제를 이용해 hello 함수에 `MyBox<String>` 값의 참조를 전달
  - deref 메서드를 호출하여 `&MyBox<String>` 을 `&String` 타입으로 변환
  - 표준 라이브러리는 String 타입에 문자열 슬라이스를 리턴하는 Deref 트레이트를 구현하고 있음
    - &String 타입의 deref 메서드를 다시 호출하여 &str 타입으로 변환

```rust
fn main() {
    let msg = MyBox::new("World");
    hello(&(*msg)[..]);
}
```
- 러스트가 역참조 강제를 지원하지 않았으면 작성했어야 할 코드

<br>

#### **🤔 역참조 강제와 가변성**
- Deref 트레이트 : 불변 참조에 대한 * 연산자의 동작을 재정의
- DerefMut 트레이트 : 가변 참조에 대한 * 연산자의 동작을 재정의
- `T: Deref<Target=U>` : &T를 &U로 변환
  - 만일 &T 타입을 가진 경우 T가 어떤 타입 U를 위한 Deref 트레이트를 구현한다면 &U 참조에 대한 투명성을 가진다는 뜻
- `T: DerefMut<Target=U>` : &mut T를 &mut U로 변환
  - 가변 참조에 대해서는 같은 역참조 강제가 일어난다는 뜻
- `T: Deref<Target=U>` : &mut T를 &U로 변환
  - 가변 참조를 불변 참조로 강제 변환하는 것은 가능하지만 그 반대는 불가능함
    - 대여 규칙 때문에 가변 참조는 반드시 어떤 데이터만을 참조해야 함
    - 불변 참조를 가변 참조로 변환하려면 데이터에 대한 불변 참조가 단 하나뿐이어야 하는데 대여 규칙이 이를 보장하지 않음

<br>

### **3️⃣ Drop 트레이트를 구현해서 메모리를 해제할 때 코드 실행하기**
- Drop 트레이트는 값이 범위를 벗어날 때의 동작을 재정의
  - 어떤 타입에도 구현 가능하며 파일이나 네트워크 연결 같은 자원을 해제하는 코드를 명시할 수 있음
  - 스마트 포인터를 구현할 때는 거의 항상 Drop 트레이트의 기능을 사용
- 러스트는 값이 범위를 벗어날 때 호출되는 코드를 직접 명시할 수도 있고 컴파일러가 자동으로 코드를 삽입할 수도 있음
  - 특정 타입의 사용을 마칠 때마다 매번 자원을 해제하는 코드를 주의해서 삽입할 필요가 없음
  - **자원의 누수는 절대 발생하지 않음**
- Drop 트레이트는 self의 가변 참조를 전달받는 drop 메서드를 구현할 것을 요구함

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointer의 데이터 '{}'를 해제합니다.",
            self.data);
    }
}

fn main() {
    let c = CustomSmartPointer{
        data: String::from("My Data")
    };
    let d = CustomSmartPointer{
        data: String::from("Your Data")
    };
    println!("CustomSmartPointer를 생성했습니다.");
}
// Result
// CustomSmartPointer를 생성했습니다.
// CustomSmartPointer의 데이터 'Your Data'를 해제합니다.
// CustomSmartPointer의 데이터 'My Data'를 해제합니다.
```
- 자원 해제 코드를 실행하는 Drop 트레이트를 구현한 CustomSmartPointer 구조체
- 인스턴스가 범위를 벗어나면 drop 메서드를 자동으로 호출해서 작성된 코드를 실행함
  - 자원의 해제는 생성과 반대 순서로 이루어짐

<br>

#### **🤔 std::mem::drop 함수로 값을 미리 해제하기**
- drop 함수의 자동 호출을 비활성화하기는 쉽지 않음
  - 메모리의 해제를 자동으로 처리하기 위한 것이기 때문
- 어떤 값을 미리 해제하고 싶을 경우
  - 스마트 포인터를 이용해 락(lock)을 관리하는 때
  - 같은 범위 내의 다른 코드가 락을 얻을 수 있도록 락을 가능한 빨리 해제하려는 경우
  - 러스트는 Drop 트레이트의 drop 메서드를 직접 호출하는 것을 지원하지 않음
  - 그 대신 값이 범위를 벗어나기 전에 해제하려면 std::mem::drop 함수를 호출해야 함

```rust
fn main() {
    let c = CustomSmartPointer{
        data: String::from("My Data")
    };
    println!("CustomSmartPointer를 생성했습니다.");
    c.drop();
    println!("CustomSmartPointer를 main 함수의 끝에 도달하기 전에 해제합니다.");
}
// Result
// explicit use of destructor method
```
- 메모리를 일찍 해제하기 위해 Drop 트레이트의 drop 메서드를 직접 호출
- drop 메서드를 직접 호출할 수 없다는 에러 발생
- 소멸자(destructor) : 인스턴스를 해제하는 역할을 실행하는 함수
  - 생성자와 비슷하며 러스트의 drop 함수는 소멸자의 종류 중 하나
- drop 함수는 범위를 벗어날 경우 값을 해제하기 위해 자동으로 호출하므로 명시적 호출이 불가능함
  - 이 동작을 허용하면 같은 값을 두 번 해제하려고 함

```rust
fn main() {
    let c = CustomSmartPointer{
        data: String::from("My Data")
    };
    println!("CustomSmartPointer를 생성했습니다.");
    drop(c);
    println!("CustomSmartPointer를 main 함수의 끝에 도달하기 전에 해제합니다.");
}
// Result
// CustomSmartPointer를 생성했습니다.
// CustomSmartPointer의 데이터 'My Data'를 해제합니다.
// CustomSmartPointer를 main 함수의 끝에 도달하기 전에 해제합니다.
```
- 값이 범위를 벗어나기 전에 std::mem::drop 함수를 호출해서 값 해제

<br>

### **4️⃣ `Rc<T>`, 참조 카운터 스마트 포인터**
- 러스트는 다중 소유권을 지원하기 위해 참조 카운터(reference counting)의 약어를 따온 `Rc<T>` 타입을 지원함
  - 값에 대한 참조의 개수를 추적하여 그 값이 여전히 사용 중인지 확인함
  - 값에 대한 참조가 더 존재하지 않을 시 값을 해제함
- `Rc<T>` 타입은 프로그램의 여러 부분에서 데이터를 읽을 수 있도록 데이터를 힙 메모리에 저장할 때 사용하며, 컴파일 타임에는 어떤 곳의 데이터가 가장 마지막까지 사용되는지 결정할 수 없음
- `Rc<T>` 타입은 단일 스레드(single-threaded) 환경에서만 사용해야 함

<br>

#### **🤔 `Rc<T>` 타입을 이용해 데이터 공유하기**

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Cons(5, 
        Box::new(Cons(10, 
            Box::new(Nil))));
    
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
// Result
// use of moved value: `a`
```
- `Box<T>` 타입을 이용해서는 세 번째 리스트의 소유권을 공유할 수 없음을 확인
  - Cons 열것값은 자신이 저장한 데이터를 소유하고 있으므로 b 리스트를 생성할 때 a가 b로 이동해서 b가 a를 소유함
  - 다시 c 리스트를 생성하는 코드는 a가 이미 이동했기에 동작하지 않음
- Cons 구조체가 참조를 저장하도록 변경할 수는 있지만 그렇게 하면 수명 매개변수를 지정해야 함
  - 수명 매개변수 지정 시 리스트의 모든 원소가 전체 리스트와 같은 수명을 가지게 할 수 있음
    - 대여 검사기는 `let a = Cons(10, &Nil);` 코드의 컴파일을 허용하지 않음 -> a가 참조를 얻기 전에 Nil 값이 해제될 수 있어서
- List 정의에서 `Box<T>` 대신 `Rc<T>` 타입 사용
  - 모든 Cons 열것값은 값과 더불어 `Rc<T>` 타입이 가리키는 List 타입을 저장함
  - b 리스트 생성 시 a의 소유권이 아니라 a가 가지고 있는 `Rc<List>` 를 복제 -> 참조의 개수가 한 개에서 두 개로 늘어나 a와 b가 `Rc<List>` 의 데이터에 대한 소유권을 공유함
  - c 리스트 생성 시에도 복제가 발생하여 참조 개수가 두 개에서 세 개로 늘어남
  - `Rc::clone` 을 호출할 때마다 `Rc<List>` 안에 데이터에 대한 참조 카운트가 증가하고 이 참조가 모두 사라질 때까지 데이터는 해제되지 않음

```rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```
- `Rc<T>` 타입은 프렐류드에 포함되어 있지 않음
- `Rc::clone(&a)` 대신 `a.clone()` 함수를 호출해도 되지만 러스트의 규칙은 `Rc::clone(&a)` 를 호출하는 것이 관례
  - `Rc::clone` 함수는 대부분 타입이 지원하는 clone 메서드와 마찬가지로 모든 데이터에 대한 깊은 복사를 수행하지 않음
    - 참조 카운터만 증가하므로 빠르게 실행되지만, 데이터의 깊은 복사는 많은 시간이 소요됨

<br>

#### **🤔 `Rc<T>` 의 복제는 참조 카운터를 증가시킨다**
- 