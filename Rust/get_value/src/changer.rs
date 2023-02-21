pub mod operations {
    pub use std::collections::HashMap;

    pub fn make_integers() -> Vec<i32> {
        let mut n: Vec<i32> = Vec::new();
        let mut nums: [i32; 17] = [20, 17, 114, 24, 14, 170, 141, 67, 20, 4, 78, 404, 19, 67, 221, 67, 20];
        
        for i in &mut nums {
            *i += 1;
            n.push(*i);
        }

        n
    }

    pub fn get_mode(nums: &Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut result: HashMap<i32, i32> = HashMap::new();
        let mut tmp = 0;
        let mut tmp2 = 0;
    
        for i in nums {
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }
    
        for (key, value) in &map {
            match result.is_empty() {
                true => {
                    result.insert(**key, *value);
                    tmp = **key;
                },
                false => {
                    match result.get(&tmp) {
                        Some(n) => { tmp2 = *n; },
                        None => { },
                    }
                    if value == &tmp2 {
                        result.insert(**key, *value);
                        tmp = **key;
                    } else if value < &tmp2 {
                        continue;
                    } else {
                        result.clear();
                        result.insert(**key, *value);
                        tmp2 = *value;
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