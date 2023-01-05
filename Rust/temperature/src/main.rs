use std::io;

// 변수의 타입을 출력하는 함수
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn input_temperature() -> f64 {
    loop{
        println!("온도를 입력해주세요...");

        let mut tem = String::new();
        io::stdin().read_line(&mut tem)
            .expect("Unable to read input!");

        let tem: f64 = match tem.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break tem;
    }
}

fn calculation(u: &str, t: f64) -> f64 {
    let mut result: f64 = 0.0;

    if u=="C"||u=="c" {
        result = (t*1.8)+32.0;
    } else if u=="F"||u=="f" {
        result = (t-32.0)/1.8;
    }

    return result;
}

fn main() {

    // // alloc::string::String
    // let test_string = String::from("test!!!");
    // print_type_of(&test_string);
    // println!("{}", test_string);

    // // &str
    // let test_str = "test!!!";
    // print_type_of(&test_str);
    // println!("{}", test_str);

    // // &str 슬라이싱
    // let test_slice = &test_string[1..4];
    // print_type_of(&test_slice);
    // println!("{}", test_slice);

    loop{
        println!("단위 입력(C & F)...");

        let mut unit = String::new();
        io::stdin().read_line(&mut unit)
            .expect("Unable to read input!");

        let unit: &str = unit.trim();

        if unit=="C"||unit=="c" {
            println!("Celsius");
            let result: f64 = calculation(unit, input_temperature()).round();
            println!("{}F", result);
            break;
        } else if unit=="F"||unit=="f" {
            println!("Fahrenheit");
            let result: f64 = calculation(unit, input_temperature()).round();
            println!("{}C", result);
            break;
        } else {
            println!("다시 입력하세요...");
            continue;
        }
    }
}