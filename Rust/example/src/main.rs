// 변수의 타입을 출력하는 함수
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // alloc::string::String
    let test_string = String::from("test!!!");
    print_type_of(&test_string);
    println!("{}", test_string);

    // &str
    let test_str = "test!!!";
    print_type_of(&test_str);
    println!("{}", test_str);

    // 문자열 슬라이싱
    let test_slice = &test_string[1..4];
    print_type_of(&test_slice);
    println!("{}", test_slice);
}