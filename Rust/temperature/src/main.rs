use std::io;

// 온도 입력 함수
fn input_temperature() -> f64 {
    loop{
        println!("온도를 입력해주세요...");

        let mut tem = String::new();
        io::stdin().read_line(&mut tem)
            .expect("Unable to read input!");

        let tem: f64 = match tem.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error! You have to input number!");
                continue
            },
        };

        break tem;
    }
}

// 계산 함수
fn calculation(u: &str, t: f64) -> f64 {
    let mut result: f64 = 0.0;

    if u=="C"||u=="c" {
        result = (t*1.8)+32.0;
    } else if u=="F"||u=="f" {
        result = (t-32.0)/1.8;
    }   

    return result;
}

// 메인 함수
fn main() {
    loop{
        println!("단위 입력(C & F)...");

        // 입력한 값을 문자열로 저장하기 위한 변수 초기화
        let mut unit = String::new();

        // 사용자 입력을 위한 코드
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