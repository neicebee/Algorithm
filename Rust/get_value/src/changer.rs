pub mod operations {
    pub use std::collections::HashMap;

    // vec! 매크로를 쓰고 싶지 않은 마음에 작성한 함수
    pub fn make_integers() -> Vec<i32> {
        // 배운 것은 써먹으라 있는 것.
        let mut n: Vec<i32> = Vec::new();
        let mut nums: [i32; 17] = [20, 17, 114, 24, 14, 170, 141, 67, 20, 4, 78, 404, 19, 67, 221, 67, 20];
        
        // 역참조 연산자를 이용하여 nums의 요소들에 1을 더한 한 값을 벡터에 push
        for i in &mut nums {
            *i += 1;
            n.push(*i);
        }
        
        // 벡터 정렬
        n.sort();
        
        n
    }

    // 최빈값을 구하는 함수
    pub fn get_mode(nums: &Vec<i32>) -> Vec<i32> {
        // nums의 요소들의 빈도수를 매치하는 HashMap 인스턴스
        let mut map = HashMap::new();
        // 최빈값을 골라내 저장할 용도인 HashMap 인스턴스
        let mut result: HashMap<i32, i32> = HashMap::new();
        // for 루프에서 현재 키에 연결된 값과 비교하기 위해 키 값을 저장할 임시적인 변수
        let mut tmp = 0;
        // tmp의 값과 비교 후 결과로 나온 키의 값을 저장할 임시적인 변수
        let mut tmp2 = 0;
        
        // nums 요소들에 대한 빈도수를 HashMap 인스턴스에 저장
        for i in nums {
            // entry() => 값의 존재 여부를 알려주는 Entry 열거자를 반환함
            // or_insert() => Entry 열거자에 선언된 메서드
                // 키 존재 시 해당 키에 연결된 값에 대한 가변 참조로 반환함
                // 키 존재하지 않을 시 매개변수로 전달한 키에 새로운 값을 추가한 후 해당 값에 대한 가변 참조를 반환함
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }
        
        // map의 요소들을 한 번씩 돌며 비교하는 for 루프
        for (key, value) in map {
            // 해당 컬렉션이 비어있는지를 bool 타입으로 반환하는 is_empty 메서드
            // 때문에 match 표현식 사용
            match result.is_empty() {
                // result가 비어있을 경우 현재 키와 값을 insert 후 tmp에 키를 저장
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
        result.into_keys().collect()
    }

    pub fn get_avg(nums: &Vec<i32>) -> f64 {
        let mut tmp: i32 = 0;
        let mut count = 0;
        let mut avg: f64 = 0.0;
        
        for i in nums {
            tmp += i;
            count += 1;
            if i == &nums[nums.len()-1] {
                avg = f64::from(tmp);
                avg = avg/f64::from(count);
            }
        }
    
        avg
    }
}