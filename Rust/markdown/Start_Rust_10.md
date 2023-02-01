# 🦀 Rust Day 10

## **🏳️ Using Structs to Structure Related Data**
- 구조체(struct 또는 structure)는 서로 관련이 있는 여러 값을 의미 있는 하나로 모으고, 이름을 지정해 접근할 수 있는 사용자 정의 데이터 타입
- 구조체는 객체 지향 언어의 _'객체의 데이터 속성'_ 과 같다고 생각해도 무방

### **1️⃣ 구조체 정의와 인스턴스 생성**
- 튜플과 유사함
- 각 데이터에 별개의 이름을 부여해서 값의 의미를 분명하게 표현할 수 있음
- 각 데이터에 이름이 있으므로 튜플보다 유연함
  - 참조할 데이터를 가리키거나 인스턴스의 값을 읽을 때 데이터의 순서에 의존할 필요가 없기 때문
- **😎 구조체 정의**

```rust
struct User {
    username: String,
    pw: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}
```
- `struct` keyword 다음에 구조체에 부여할 이름 지정
  - _이름은 그룹화된 데이터를 잘 표현할 수 있어야 함_
- 중괄호 내에 구조체가 저장할 데이터 타입과 이름 나열
  - 데이터들은 `필드(field)` 라 칭함
- **😎 구조체 인스턴스 생성**

```rust
let user1 = User {
        username: String::from("f1r3_r41n"),
        pw: String::from("a12345"),
        email: String::from("qkrghkql1@gmail.com"),
        sign_in_count: 3,
        active: true,
    };
```
- 해당 구조체의 이름과 중괄호를 이용해 `'key: value'` 의 쌍을 나열
  - `key` : 필드의 이름
  - `value` : 해당 필드에 저장할 데이터
- 필드의 나열 순서는 구조체에 정의한 순서와 반드시 같을 필요는 없음
- **_구조체의 정의는 타입의 템플릿_**
- **_인스턴스는 해당 타입의 값을 생성하기 위해 템플릿에 값을 채운 것_**
- **😎 인스턴스 활용**

```rust
#[derive(Debug)]
struct User {
    username: String,
    pw: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

fn main() {
    let id = String::from("f1r3_r41n");
    let pw = String::from("a12345");
    let email = String::from("qkrghkql1@gmail.com");

    let user1 = build_user(id, pw, email);

    println!("{}", user1.username);
    println!("{:?}", user1);
}

fn build_user(username: String, pw: String, email: String) -> User {
    User{
        username: username,
        pw: pw,
        email: email,
        sign_in_count: 3,
        active: true,
    }
}
// Result
// f1r3_r41n
// User { username: "f1r3_r41n", pw: "a12345", email: "qkrghkql1@gmail.com", sign_in_count: 3, active: true }
```
- 구조체에서 원하는 값을 읽으려면 `마침표(.)` 사용
  - `user1.username`
- 인스턴스가 가변 인스턴스인 경우 마침표를 사용해 특정 필드에 새로운 값 대입 가능
  - 구조체의 인스턴스 자체가 가변이어야 함
  - **_러스트는 구조체의 몇몇 필드만을 가변 데이터로 표시하는 것을 지원하지 않음_**
- 함수를 이용해 새로운 인스턴스 생성 가능
  - 함수의 마지막 표현식은 묵시적으로 새 인스턴스를 리턴해야 함

#### **🤔 같은 이름의 필드와 변수를 편리하게 활용하기**

```rust
fn build_user(username: String, pw: String, email: String) -> User {
        User{
            username,
            pw,
            email: email,
            sign_in_count: 3,
            active: true,
        }
}
```
- 함수의 매개변수 이름과 구조체의 필드 이름이 같을 경우
  - `필드 초기화 단축 문법(field init shorthand syntax)` 사용
- 인스턴스 활용 부분의 코드 중 `build_user` 함수에 `필드 초기화 단축 문법` 을 적용한 코드

#### **🤔 기존의 인스턴스로부터 새 인스턴스 생성하기**

```rust
#[derive(Debug)]
struct User {
    username: String,
    pw: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

fn main() {
    let user1 = User{
        username: String::from("f1r3_r41n"),
        pw: String::from("a12345"),
        email: String::from("qkrghkql1@gmail.com"),
        sign_in_count: 3,
        active: true,
    };

    let user2 = User{
        username: String::from("gnuykob_"),
        pw: String::from("b12345"),
        email: String::from("leebk1124@gmail.com"),
        ..user1
    };

    println!("{:?}", user1);
    println!("{:?}", user2);
}
// Result
// User { username: "f1r3_r41n", pw: "a12345", email: "qkrghkql1@gmail.com", sign_in_count: 3, active: true }
// User { username: "gnuykob_", pw: "b12345", email: "leebk1124@gmail.com", sign_in_count: 3, active: true }
```
- 이미 존재하는 인스턴스에서 몇 가지 필드의 값만 수정한 상태의 새 구조체 인스턴스가 필요한 경우
  - `구조체 갱신 문법(struct update syntax)` 사용
- `..` 문법은 명시적으로 나열하지 않은 나머지 필드에 대해서는 기존의 인스턴스 필드와 같은 값을 사용하라는 의미 내포

#### **🤔 이름 없는 필드를 가진 튜플 구조체로 다른 타입 생성하기**
- 튜플과 유사하게 생긴 구조체 선언 가능
  - `튜플 구조체(tuple structs)`
- 구조체에는 이름을 부여하지만, 필드에는 이름을 부여하지 않고 타입만 지정
- 일반적인 구조체처럼 필드에 이름을 부여하는 것이 귀찮거나 불필요하고, 튜플 자체에만 이름을 부여해 여타의 튜플들과는 다른 타입으로 구분하고자 할 때 유용함
- **😎 튜플 구조체 정의**
  - `struct` keyword와 구조체 이름, 튜플 안에서 사용할 타입들을 차례로 나열
  - 같은 타입들을 포함하는 각각의 튜플 구조체의 인스턴스는 서로 다른 타입
    - '`B` 튜플 구조체 인스턴스' 와 같은 타입을 포함한 '`A` 튜플 구조체 인스턴스' 를 매개변수로 사용하는 함수에 '`B` 튜플 구조체 인스턴스' 를 인수로 전달 불가능
  - 위의 점만 제외하면 튜플과 똑같이 동작함
    - **(1).** 각 필드값에 대응하는 변수로 해체 가능
    - **(2).** `마침표(.)` 와 인덱스를 이용해 개별 필드값에 접근 가능

#### **🤔 필드가 없는 유사 유닛 구조체**
- 필드가 하나도 없는 구조체 선언 가능
  - `유사 유닛 구조체(unit-like structs)`
  - 이 구조체는 유닛 타입, 즉 `()` 과 유사하게 동작하기 때문
- 유사 유닛 구조체는 어떤 타입의 트레이트를 구현해야 하지만, 타입에 저장할 데이터가 없을 때 유용하게 활용할 수 있음
- **😎 구조체 데이터의 소유권**
  - 이 글에서 구조체 정의 시 `&str` 문자열 슬라이스 타입이 아닌 `String` 타입을 사용하고 있음
    - 구조체가 데이터의 소유권을 갖게 함으로써 유효한 범위 내에 존재하는 동안 그 데이터도 유효할 수 있도록 하기 위한 의도임
  - _구조체에 다른 변수가 소유한 데이터의 참조를 저장할 수 있음_
    - `수명(lifetimes)` 를 사용해야 함
    - 수명은 구조체의 유효 범위 안에서 구조체가 참조하는 데이터가 유효하도록 보장해 줌
    - 수명을 지정하지 않고 구조체에 참조를 저장하면 정상적으로 동작하지 않음
  - 아직 수명에 대해 살펴보지 않았기에 `String` 타입과 같이 소유할 수 있는 타입을 사용할 것임

### **2️⃣ 구조체를 사용하는 예제 프로그램**
- 사각형의 면적을 구하는 프로그램 작성
  - 변수를 이용해 프로그램 작성 후 구조체 사용 형태로 리팩토링

#### **🤔 변수 이용**

```rust
fn main() {
    let width1: u32 = 30;
    let height1: u32 = 50;

    println!(
        "사각형 1의 면적 : {} 제곱 픽셀", 
        get_area(width1, height1)
    );
}

fn get_area(width: u32, height: u32) -> u32 {
    width * height
}
// Result
// 사각형 1의 면적 : 1500 제곱 픽셀
```
- `get_area` 함수는 한 사각형의 면적을 구하지만, 두 개의 매개변수를 선언하고 있음
- 너비와 높이를 연관지을 수 있다면 **_프로그램의 가독성이 향상되는 것_** 은 물론 **_관리도 쉬울 것_** 임

#### **🤔 튜플 이용**

```rust
fn main() {
    let width1: u32 = 30;
    let height1: u32 = 50;

    println!(
        "사각형 1의 면적 : {} 제곱 픽셀", 
        get_area((width1, height1))
    );
}

fn get_area(compression: (u32, u32)) -> u32 {
    compression.0 * compression.1
}
// Result
// 사각형 1의 면적 : 1500 제곱 픽셀
```
- 튜플은 요소에 이름을 부여하지 않음
  - **_각 요소에 의미가 명확하지 않음_**

#### **🤔 구조체 이용 : 더 많은 의미 반영하기**

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle1 = Rectangle{ width: 30, height: 50, };
    let rectangle2 = Rectangle{ width: 31, height: 45, };

    println!(
        "사각형 1의 면적 : {} 제곱 픽셀",
        get_area(&rectangle1)
    );
    println!(
        "사각형 2의 면적 : {} 제곱 픽셀",
        get_area(&rectangle2)
    );
}

fn get_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
// Result
// 사각형 1의 면적 : 1500 제곱 픽셀
// 사각형 2의 면적 : 1395 제곱 픽셀
```
- 데이터에 이름을 부여해서 의미를 반영하기 위해 구조체 사용
- `get_area` 함수는 하나의 매개변수만을 사용하고 있음
  - 매개변수 타입은 **_구조체의 불변 인스턴스에 대한 대여_**
  - 대여를 사용하면 `main` 함수가 해당 변수의 소유권을 계속 갖고 있으므로 추가적으로 범위 내에서 사용할 수 있음

#### **🤔 트레이트를 상속해서 유용한 기능 추가하기**
- 구조체 인스턴스가 가진 값들을 확인할 수 있다면 디버깅에 매우 효율적일 것임
- `println!("rectangle1 : {}", rectangle1);` $=>$ `Error`
- **😎 `println` 매크로**
  - 다양한 형식으로 문자열 출력 가능
  - 중괄호는 기본적으로 해당 매크로가 `Display` 형식을 출력하도록 함
    - `Display` 형식 : 최종 사용자를 위한 출력 형식
  - 기본 타입들은 자신을 표현할 방법이 단 한 가지밖에 없기에 `Display` 형식이 기본적으로 구현되어 있음
  - 구조체와 비슷한 웬만한 컴파운드 타입들은 자신을 출력할 방법이 하나 이상임
  - 러스트는 연속된 값을 갖는 타입에 대해 `Display` 트레이트를 구현하지 않도록 남겨둠
  - `println` 매크로의 중괄호 안에 `:?` 연산자를 지정하면 `Debug` 라는 출력 형식을 이용해 결과를 출력함
  - `Debug` 트레이트는 개발자들에게 유용한 형식으로 구조체의 값을 출력함
  - 러스트는 디버깅 정보를 출력하는 기능을 제공하지만 구조체는 이 기능을 **명시적**으로 구현해주어야 함
  - `#[derive(Debug)]` 애노테이션을 구조체 정의에 추가
  - 구조체에 포함된 필드의 수가 많다면 `:?` 대신 `:#?` 사용
- 러스트는 `derive` 애노테이션을 이용해 사용자 정의 타입에 유용한 동작을 적용할 수 있는 다양한 트레이트 제공

### **3️⃣ 메서드 문법**
- `메서드(method)` 는 함수와 유사함
- `fn` keyword로 정의하며, 이름, 매개변수, 리턴 타입을 정의 가능
- 호출 시 실행할 일련의 코드도 정의 가능
- 함수와 달리 구조체의 `컨텍스트(context)` 안에 정의하며, 첫 번째 매개변수는 항상 메서드를 호출할 구조체의 인스턴스를 표현하는 `self` 이어야 함
  - **_메서드는 열거자나 트레이트 객체에도 정의 가능_**

#### **🤔 메서드 정의하기**

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rectangle1 = Rectangle{ width: 30, height: 50, };
    let rectangle2 = Rectangle{ width: 31, height: 45, };

    println!(
        "사각형 1의 면적 : {} 제곱 픽셀",
        rectangle1.get_area()
    );
    println!(
        "사각형 2의 면적 : {} 제곱 픽셀",
        rectangle2.get_area()
    );
}
// Result
// 사각형 1의 면적 : 1500 제곱 픽셀
// 사각형 2의 면적 : 1395 제곱 픽셀
```
- 구조체 타입의 컨텍스트 안에 함수 정의 시 `impl(implementation)` 블록 이용
- 메서드 문법 사용으로 `get_area` 메서드는 `Rectangle` 인스턴스 상에서 호출 가능
- 메서드 문법은 인스턴스 다음에 따라옴
  - 인스턴스 다음에 마침표, 메서드의 이름, 괄호, 필요한 인수를 전달하면 됨
- `get_area` 메서드의 시그니처가 `&self` 임
  - 함수가 `Rectangle` 구조체의 컨텍스트 안에 존재하므로 러스트는 `self` 의 타입이 `Rectangle` 이라는 것을 알고 있음
  - 메서드는 `self` 에 대한 소유권을 갖거나 `self` 의 불변 인스턴스를 대여하거나 `self` 의 가변 인스턴스를 대여할 수 있음
- 해당 코드에서는 단순히 구조체의 데이터를 읽을 뿐 값을 쓰지 않기 때문에 굳이 소유권을 가질 필요가 없음
  - **참조의 사용 이유**
- 메서드를 호출한 인스턴스의 값을 변경하고자 한다면 매개변수를 `&mut self` 로 선언해야 함
  - 매개변수를 `self` 로만 선언하여 인스턴스에 대한 소유권을 갖는 메서드는 드묾
  - 메서드가 `self` 를 다른 인스턴스로 교체한 후 호출자가 더 이상 예전 인스턴스를 사용하지 못하도록 할 경우에 활용
- **😎 함수 대신 메서드를 사용할 때 장점**
  - **(1).** 호출 때마다 매개변수로 타입을 넘겨줄 필요가 없음
  - **(2).** 코드를 더 잘 정리할 수 있음
- **😎 `->` 연산자는 어디에?**
  - C나 CPP에서는 메서드를 호출할 때 서로 다른 연산자 사용
    - 객체의 메서드를 직접 호출할 때 `.` 연산자 사용
    - 객체의 포인터를 이용해 메서드를 호출 시 `->` 연산자를 사용하거나 포인터를 `역참조(dereference)` 해야 함
    - 객체가 포인터일 때는 `object->something()` 은 `(*object).something()` 과 유사함
  - 러스트에는 `->` 연산자에 해당하는 연산자가 없음
    - 러스트는 `자동 참조 및 역참조(automatic referencing and dereferencing)` 기능 제공
    - 메소드의 호출 역시 러스트가 이 기능을 확용하는 부분 중 하나
  - `object.something()` 과 같이 메서드 호출 시 러스트는 메서드의 시그니처에 따라 자동으로 `object` 변수에 `&` , `&mut` , 또는 `*` 을 추가함
  - `p1.distance(&p2);` $=$ `(&p1).distance(&p2);`
  - 자동 참조 기능은 메서드가 수신자, 즉 `self` 의 타입을 명확하게 선언하기에 동작하는 것임
  - 메서드의 수신자와 이름 덕분에 러스트는 이 메서드가 값을 읽는지 `(&self)` , 값을 쓰는지 `(&mut self)` , 아니면 소비하는지 `(self)` 를 정확히 알 수 있음

#### **🤔 더 많은 매개변수를 갖는 메서드**

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width>other.width && self.height>other.height
    }
}

fn main() {
    let rectangle1 = Rectangle{ width: 30, height: 50, };
    let rectangle2 = Rectangle{ width: 10, height: 40, };
    let rectangle3 = Rectangle{ width: 60, height: 45, };

    println!(
        "사각형 1의 면적 : {} 제곱 픽셀",
        rectangle1.get_area()
    );
    println!(
        "사각형 2의 면적 : {} 제곱 픽셀",
        rectangle2.get_area()
    );
    println!(
        "사각형 1이 사각형 2를 포함하는가? {}", 
        rectangle1.can_hold(&rectangle2)
    );
    println!(
        "사각형 1이 사각형 3을 포함하는가? {}", 
        rectangle1.can_hold(&rectangle3)
    );
}
// Result
// 사각형 1의 면적 : 1500 제곱 픽셀
// 사각형 2의 면적 : 400 제곱 픽셀
// 사각형 1이 사각형 2를 포함하는가? true
// 사각형 1이 사각형 3을 포함하는가? false
```
- 구조체의 인스턴스에 또 다른 인스턴스를 전달해서 첫 번째 인스턴스의 면적이 두 번째 인스턴스의 면적을 완전히 포함할 수 있다면 `true` 리턴, 그렇지 않으면 `false` 를 리턴하는 코드
- 메서드에 여러 개의 매개변수를 사용하려면 `self` 매개변수 이후에 원하는 만큼의 매개변수를 추가하면 됨

#### **🤔 연관 함수**
- `impl` 블록은 `self` 매개변수를 사용하지 않는 다른 함수로 정의할 수 있음 => `연관 함수(associated functions)`
- 구조체의 인스턴스를 직접 전달받지 않기에 메서드가 아닌 함수임
  - Ex) `String::from` 함수
- 연관 함수는 구조체의 새로운 인스턴스를 리턴하는 `생성자(constructors)` 를 구현할 때 자주 사용

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let square1 = Rectangle::square(4);
    println!(
        "width: {}\nheight: {}", 
        square1.width, square1.height
    );
}
// Result
// width: 4
// height: 4
```
- 연관 함수를 호출하려면 구조체의 이름과 `::` 문법을 사용하면 됨
- 연관 함수는 해당 구조체에 대해서만 사용할 수 있음
- `::` 문법은 연관 함수의 호출뿐만 아니라 모듈이 생성하는 `이름 공간(namespace)` 정의에도 사용됨

#### **🤔 여러 개의 impl 블록**
- 각 구조체는 여러 개의 `impl` 블록 선언 가능
- 제네릭 타입과 트레이트에서 유용하게 활용할 수 있음

## **Summary**
- 구조체는 프로그램 내에서 특정한 의미가 있는 사용자 정의 타입을 선언하기 위한 개념
- 메서드는 구조체의 인스턴스에 원하는 동작을 부여함
- 연관 함수는 구조체의 인스턴스가 없는 상황에서 구조체에 적용할 수 있는 기능을 구분짓는 데 활용할 수 있음
- 구조체가 사용자 정의 타입을 선언하는 유일한 방법은 아님