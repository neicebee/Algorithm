use std::io;

fn main() {
    println!("랜덤 숫자를 맞혀보세요!");
    println!("예상 숫자를 입력하세요.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("입력한 값을 읽지 못했습니다.");

    println!("입력한 값: {}", guess);
}
