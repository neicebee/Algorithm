mod changer;
use crate::changer::operations;

fn main() {
    let nums: Vec<i32> = operations::make_integers();
    let mode = operations::get_mode(&nums);

    println!(
        "평균값 : {}",
        match operations::get_avg(&nums) {
            Some(avg) => avg.to_string(),
            None => "값이 없음".to_string(),
        }
    );

    println!(
        "중간값 : {}",
        match operations::get_mid(&nums) {
            Some(mid) => mid.to_string(),
            None => "값이 없음".to_string(),
        }
    );

    println!(
        "최빈값 : {}", {
            if mode.len() == 0 {
                "값이 없음".to_string()
            } else {
                let mut s = String::new();
                for i in mode {
                    s.push_str(&i.to_string());
                    s.push(' ');
                }
                s
            }
        }
    );
}