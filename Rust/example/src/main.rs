use std::collections::HashMap;

fn main() {
    let nums = vec![20, 17, 114, 24, 14, 170, 141, 67, 20, 4, 78, 404, 19, 67, 221, 67, 20];
    // nums의 요소들의 빈도수를 매치하는 HashMap 인스턴스
    let mut map = HashMap::new();
    // 최빈값을 골라내 저장할 용도인 HashMap 인스턴스
    let mut result: HashMap<i32, i32> = HashMap::new();
    // for 루프에서 현재 키에 연결된 값과 비교하기 위해 키 값을 저장할 임시적인 변수
    let mut tmp = 0;
    // tmp의 값과 비교 후 결과로 나온 키의 값을 저장할 임시적인 변수
    let mut tmp2 = 0;

    // nums 요소들에 대한 빈도수를 HashMap 인스턴스에 저장
    for i in &nums {
        // entry() => 값의 존재 여부를 알려주는 Entry 열거자를 반환함
        // or_insert() => Entry 열거자에 선언된 메서드
            // 키 존재 시 해당 키에 연결된 값에 대한 가변 참조로 반환함
            // 키 존재하지 않을 시 매개변수로 전달한 키에 새로운 값을 추가한 후 해당 값에 대한 가변 참조를 반환함
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    
    println!("{:?}", map);
    
    for (key, value) in map {
        match result.is_empty() {
            true => {
                result.insert(*key, value);
                tmp = *key;
            },
            false => {
                match result.get(&tmp) {
                    Some(n) => { tmp2 = *n; },
                    None => { },
                }
                if value == tmp2 {
                    result.insert(*key, value);
                    tmp = *key;
                } else if value < tmp2 {
                    continue;
                } else {
                    result.clear();
                    result.insert(*key, value);
                    tmp2 = value;
                }
            },
        }
    }

    let test: Vec<i32> = result.into_keys().collect();

    println!("{:?}", test);
}