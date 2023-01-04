// use std::io;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    let test_string = String::from("test!!!");
    print_type_of(&test_string);
    println!("{}", test_string);

    let test_str = "test!!!";
    print_type_of(&test_str);
    println!("{}", test_str);

    let test_slice = &test_string[1..4];
    print_type_of(&test_slice);
    println!("{}", test_slice);

    // println!("단위 입력(C & F)...");

    // // Type of unit => alloc::string::String
    // let mut unit = String::new();
    // io::stdin().read_line(&mut unit)
    //     .expect("Unable to read input!");

    // // Type of unit => &str
    // let unit: &str = &unit;

    // print_type_of(&unit);
    // print_type_of(&"Hello");

    // if unit=="C" {
    //     println!("섭씨");
    // } else if unit=="F" {
    //     println!("화씨");
    // } else {
    //     println!("다시 입력하세요...");
    // }
}