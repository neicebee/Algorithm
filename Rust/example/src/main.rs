fn main() {
    let v = vec!["일", "이", "삼", "사", "오"];
    
    for (i, v) in v.iter().enumerate() {
        println!("{i} {v}");
    }
}