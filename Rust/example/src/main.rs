use std::collections::HashMap;

fn main() {
    let s = "hello world pretty wonderful world";
    let mut map = HashMap::new();

    for word in s.split(' ') {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
}