use std::collections::HashMap;

// 직원을 해당 부서에 추가하는 함수
pub fn add_employee(map: &mut HashMap<String, Vec<String>>, query: String) -> bool {
    // 메인에서 입력한 쿼리의 소유권을 이 함수가 가짐
    // 쿼리를 공백을 기준으로 나눠 벡터로 만듬
    // 직원 추가 쿼리는 항상 일정할 것이다를 가정하면 해당 벡터의 두 번째 인덱스의 값은
    // 직원의 이름이며, 네 번째 인덱스의 값은 해당 부서임
    let split_query: Vec<&str> = query.trim().split(' ')
        .map(|x| x).collect();
    
    // 이상한 쿼리가 들어올 시 판별하여 반환할 bool
    let mut check = false;
    if split_query.len() != 4 {
        check
    } else {
        check = true;
        let k = split_query[3].to_string();
        let v = split_query[1].to_string();
        
        // 전달받은 가변 참조 HashMap을 변환하는 중점 코드
        // 값에 벡터 자체를 추가하거나 벡터에 값을 push하는 코드임
        let data = map.entry(k)
            .or_insert({
                let tmp = Vec::new();
                tmp
            });
        data.push(v);
        data.sort();
            
        check
    }
}

// 회사 내의 모든 직원들을 출력하는 함수
pub fn view_enterprise(map: &HashMap<String, Vec<String>>) {
    let mut everyone = Vec::new();

    println!("===[Enterprise]===");
    for (_, v) in map {
        let mut tmp = v.clone();
        everyone.append(&mut tmp);
    }

    everyone.sort();

    for i in everyone {
        println!("- {}", i);
    }
    println!("==================");
}

// 회사 내 각 부서의 모든 직원들을 출력하는 함수
pub fn view_department_employees(map: &HashMap<String, Vec<String>>, dpm: &String) {
    println!("===[Department]===");
    println!("- {}", dpm);
    println!("=====[Staffs]=====");
    match map.get(dpm) {
        Some(tmp) => {
            for i in tmp {
                println!("- {i}");
            }
        },
        None => (),
    }
    println!("==================")
}