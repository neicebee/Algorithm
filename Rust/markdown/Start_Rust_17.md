# 🦀 Rust Day 17

## **🏳️ Rust Example Script 5 - Simple Employees DataBase**

### **1️⃣ Description**
- 간단한 직원 정보를 `HashMap` 과 `Vector` 를 이용해 저장하는 프로그램 작성
- 이름이 `Sally` 인 직원을 `Engineering` 부서에 추가할 경우
  - `add Sally to Engineering`
- 이름이 `Amir` 인 직원을 `Sales` 부서에 추가할 경우
  - `add Amir to Sales`
- 회사 내의 모든 직원들을 알파벳 순으로 출력
- 회사 내의 각 부서의 직원들을 출력

### **2️⃣ how it works**
- 문자열 입력받기
- 상황에 맞는 결과 출력

### **3️⃣ Code**

```rust
// src/main.rs
mod command;

use command::*;
pub use std::{io, collections::HashMap};

fn main() {
    let mut enterprise: HashMap<String, Vec<String>> = HashMap::new();
    println!("**** Add Employee Information Text Interface ****");
    println!("*   Add => \"add {{name}} to {{department}}\"         *");
    println!("* Check department employees => \"{{department}}\"  *");
    println!("*  Check enterprise employees => \"enterprise\"   *");
    println!("*               Exit => \"exit\"                  *");
    println!("*************************************************");

    loop {
        let mut query = String::new();
        let mut check = 0;

        println!("Input: ");
        match io::stdin().read_line(&mut query) {
            Ok(_) => {
                if &query == "exit\n" {
                    break;
                } else if &query == "enterprise\n" {
                    view_enterprise(&enterprise);
                } else {
                    for (k, _) in &enterprise {
                        if k == &query.trim() {
                            view_department_employees(&enterprise, k);
                            check = 1;
                        }
                    }
                    if check == 0 {
                        match add_employee(&mut enterprise, query) {
                            false => {
                                println!("Input Error!");
                                continue;
                            },
                            true => {
                                println!("Complete!");
                            },
                        }
                    }
                }
            },
            Err(_) => {
                println!("Input Error!");
                continue;
            },
        }
        println!("Running...");
    }
}
```

```rust
// src/command.rs
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
```

### **4️⃣ Result**
```bash
**** Add Employee Information Text Interface ****
*   Add => "add {name} to {department}"         *
* Check department employees => "{department}"  *
*  Check enterprise employees => "enterprise"   *
*               Exit => "exit"                  *
*************************************************
Input: 
add Sally to Engineering
Complete!
Running...
Input: 
add Amir to Sales 
Complete!
Running...
Input: 
add Hwabee to Development
Complete!
Running...
Input: 
add Bokyung to Development
Complete!
Running...
Input: 
enterprise
===[Enterprise]===
- Amir
- Bokyung
- Hwabee
- Sally
==================
Running...
Input: 
Development
===[Department]===
- Development
=====[Staffs]=====
- Bokyung
- Hwabee
==================
Running...
Input: 
Engineering
===[Department]===
- Engineering
=====[Staffs]=====
- Sally
==================
Running...
Input: 
exit
```