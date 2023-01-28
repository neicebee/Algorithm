# 🦀 Rust Day 10

## **🏳️ Using Structs to Structure Related Data**
- 구조체(struct 또는 structure)는 서로 관련이 있는 여러 값을 의미 있는 하나로 모으고, 이름을 지정해 접근할 수 있는 사용자 정의 데이터 타입
- 구조체는 객체 지향 언어의 _'객체의 데이터 속성'_ 과 같다고 생각해도 무방

### **1️⃣ 구조체 정의와 인스턴스 생성**
- 튜플과 유사함
- 각 데이터에 별개의 이름을 부여해서 값의 의미를 분명하게 표현할 수 있음
- 각 데이터에 이름이 있으므로 튜플보다 유연함
  - 참조할 데이터를 가리키거나 인스턴스의 값을 읽을 때 데이터의 순서에 의존할 필요가 없기 때문
- **구조체 정의**

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
- **구조체 인스턴스 생성**

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
  - 필ㄹ드의 나열 순서는 구조체에 정의한 순서와 반드시 같을 필요는 없음
  - **_구조체의 정의는 타입의 템플릿_**
  - **_인스턴스는 해당 타입의 값을 생성하기 위해 템플릿에 값을 채운 것_**
- **인스턴스 활용**

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
- **튜플 구조체 정의**
  - `struct` keyword와 구조체 이름, 튜플 안에서 사용할 타입들을 차례로 나열
  - 같은 타입들을 포함하는 각각의 튜플 구조체의 인스턴스는 서로 다른 타입
    - '`B` 튜플 구조체 인스턴스' 와 같은 타입을 포함한 '`A` 튜플 구조체 인스턴스' 를 매개변수로 사용하는 함수에 '`B` 튜플 구조체 인스턴스' 를 인수로 전달 불가능
  - 위의 점만 제외하면 튜플과 똑같이 동작함
    - **(1).** 각 필드값에 대응하는 변수로 해체 가능
    - **(2).** `마침표(.)` 와 인덱스를 이용해 개별 필드값에 접근 가능