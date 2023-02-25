# 🦀 Rust Day 15

## **🏳️ Rust Example Script 3 - Get Avg & Mid & Mode**

### **1️⃣ Description**
- 정수 리스트가 주어졌을 때, 벡터를 이용하여 평균값, 중간값, 최빈값을 구하는 프로그램 작성

### **2️⃣ how it works**
- `i32` 타입의 정수 배열이 주어짐
- 해당 배열의 평균값, 중간값, 최빈값을 순서대로 출력

### **3️⃣ Code**

```rust
// src/main.rs
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
```

```rust
// src/changer.rs
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

    // 중간값을 구하는 함수
    pub fn get_mid(nums: &Vec<i32>) -> Option<i32> {
        if nums.len() == 0 { return None; }

        Some(nums[nums.len()/2])
    }

    // 최빈값을 구하는 함수
    pub fn get_mode(nums: &Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 { return Vec::new(); }
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
                // result가 비어있을 경우 현재 키와 값을 insert 후 포인터 역할을 하는 tmp에 키를 저장
                true => {
                    result.insert(*key, value);
                    tmp = *key;
                },
                // result가 비어있지 않을 경우 tmp에 저장되어 있는 키를 이용해 값 호출 및 비교
                false => {
                    // HashMap의 get 메서드는 Option 열거자를 반환함
                    // 때문에 match 표현식 사용
                    match result.get(&tmp) {
                        // tmp에 저장되어 있는 키로 값을 찾을 수 있다면 tmp2에는 tmp에 상응하는 값을 저장
                        Some(n) => { tmp2 = *n; },
                        None => { },
                    }

                    // 만약 현재 value가 tmp2와 같다면 result에 추가한 후 포인터를 다음으로 키로 옮겨줌
                    // value가 tmp2보다 작다면 최빈값이 될 수 없으므로 아무 작업도 하지 않고 넘어감
                    // value가 tmp2보다 크다면 그 전까지 저장했던 다른 값들도 현재 value보다 크지 않을 것이므로
                    // HashMap을 초기화하는 clear 메서드를 수행 후 해당 키와 값을 추가함
                    // tmp2는 제일 큰 값만 저장하므로 해당 값으로 수정함
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
        // 우리는 최빈값이 필요하지 얼마나 나왔는지는 중요하지 않으므로 into_keys 메서드로 키만 추출
        // 추출 후 collect 메서드로 묶어줌
        result.into_keys().collect()
    }

    // 평균값을 구하는 함수
    pub fn get_avg(nums: &Vec<i32>) -> Option<f64> {
        if nums.len() == 0 { return None; }
        // 전체 합을 저장하는 변수
        let mut tmp: i32 = 0;
        
        for i in nums {
            tmp += i;
        }
    
        Some(tmp as f64 / nums.len() as f64)
    }
}
```

### **4️⃣ Result**

```rust
평균값 : 87.29411764705883
중간값 : 68
최빈값 : 21 68
```