use std::{rc::Rc, cell::RefCell};
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    // Cons 열것값이 가리키는 List 값을 수정할 수 있도록 RefCell<Rc<List>> 사용
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // Cons 열것값에 저장된 두 번째 원소에 쉽게 접근하기 위한 tail 메서드
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match *self {
            Cons(_, ref item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // 초깃값 5와 Nil을 갖는 List 값을 저장한 Rc<List> 인스턴스
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a의 최초 rc 카운트 = {}", Rc::strong_count(&a));
    println!("a의 다음 아이템 = {:?}", a.tail());

    // 값 10과 a에 저장한 리스트에 대한 포인터를 저장하는 다른 List 값을 갖는 Rc<List> 인스턴스
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("b를 생성한 후 a의 rc 카운트 = {}", Rc::strong_count(&a));
    println!("b의 최초 rc 카운트 = {}", Rc::strong_count(&b));
    println!("b의 다음 아이템 = {:?}", b.tail());

    // a의 tail 메서드를 호출하여 RefCell<Rc<List>>에 대한 참조를 얻어와 link 변수에 저장
    // RefCell<Rc<List>> 타입의 borrow_mut 메서드를 호출하여 Rc<List> 안에 저장되었던 Nil 값을
    // b에 저장된 Rc<List> 값으로 변경 
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a를 변경한 후 b의 rc 카운트 = {}", Rc::strong_count(&b));
    println!("a를 변경한 후 a의 rc 카운트 = {}", Rc::strong_count(&a));

    // 순환 참조가 생성된 것을 확인하려면 다음 코드의 주석을 해제하면 됨.
    // 하지만 그렇게 하면 스택 오버플로가 발생함.
    // println!("a의 다음 아이템 = {:?}", a.tail());
}
// Result
// a의 최초 rc 카운트 = 1
// a의 다음 아이템 = Some(RefCell { value: Nil })
// b를 생성한 후 a의 rc 카운트 = 2
// b의 최초 rc 카운트 = 1
// b의 다음 아이템 = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
// a를 변경한 후 b의 rc 카운트 = 2
// a를 변경한 후 a의 rc 카운트 = 2