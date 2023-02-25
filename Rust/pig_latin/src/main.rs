use std::io;

// 모음에 대한 상수 VOWEL 선언
const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).
        expect("No input!");

    println!(
        "{}",
        // 반환받은 bool로 각 상황에 따라 출력하는 match 표현식
        match compare(&input) {
            false => {
                format!("{}-{}ay", 
                String::from(&input.trim()[1..]), &input[..1])
            },
            true => {
                format!("{}-hay", 
                input.trim())
            },
        }
    );
}

// 문자열의 첫 번째 자리가 자음인지 모음인지 상수와 비교하는 함수
fn compare(msg: &String) -> bool {
    // 변수 f_word는 문자열 슬라이싱을 사용해 &str 타입
    let f_word = &msg[..1];
    let mut dis = false;

    for c in &VOWEL {
        // 변수 c는 &char 타입이기에 to_string() 메서드로 타입 변경이 필요함
        if c.to_string() == f_word {
            dis = true;
            break;
        }
    }

    dis
}