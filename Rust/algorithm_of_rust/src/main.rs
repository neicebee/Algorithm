use std::io;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.pop(); // trim()과 비슷한 작업을 함
    buf
}

fn main() {
    let s = input();

    // 1~6까지의 눈을 가진 주사위 각 한 개씩 생성하는 코드
    let mut dices = [0; 7];
    s.split(' ').for_each(|x| dices[x.parse::<usize>().unwrap()] += 1);
    
    let reward = match dices.iter().zip(0..7).max().unwrap() {
        (3, m) => 10000+m*1000,
        (2, m) => 1000+m*100,
        (_, m) => m*100,
    };
    print!("{reward}");
}