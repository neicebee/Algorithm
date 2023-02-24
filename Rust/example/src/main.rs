use std::collections::HashMap;

fn main() {
    let integers = vec![20, 17, 114, 24, 14, 170, 141, 67, 20, 4, 78, 404, 19, 67, 221, 67, 20];

    println!("mean: {}", match mean(&integers) {
        Some(result) => result.to_string(),
        None => "No Data".to_string(),
    });

    println!("median: {}", match median(&integers) {
        Some(result) => result.to_string(),
        None => "No Data".to_string(),
    });

    let m = mode(&integers);
    println!("mode:{}", {
        if m.len() == 0 {
            " No Data".to_string()
        } else {
            let mut temp = String::new();
            for i in m {
                temp = format!("{} {}", temp, i.to_string());
            }
            temp
        }
    });

}

fn mean(ints: &Vec<i32>) -> Option<f64> {
    if ints.len() == 0 {
        return None;
    }

    let mut result = 0;

    for i in ints {
        result += i;
    }

    Some(result as f64 / ints.len() as f64)
}

fn median(ints: &Vec<i32>) -> Option<i32> {
    if ints.len() == 0 {
        return None;
    }

    let mut sorted = ints.clone();
    sorted.sort();

    Some(sorted[ints.len()/2])
}

fn mode(ints: &Vec<i32>) -> Vec<i32> {
    if ints.len() == 0 {
        return vec![];
    }

    let mut result: Vec<i32> = Vec::new();
    let mut frequencies = HashMap::new();
    let mut max = 0;

    for i in ints {
        let count = frequencies.entry(i).or_insert(0);
        *count += 1;
    }

    for (_int, frequency) in &frequencies {
        if *frequency > max {
            max = *frequency;
        }
    }

    for (int, frequency) in &frequencies {
        if *frequency == max {
            result.push(**int);
        }
    }
  
    result
}