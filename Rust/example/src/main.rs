fn get_i32_max(num_list: &[i32]) -> i32 {
    let mut max = num_list[0];
    
    for number in num_list {
        if number > &max {
            max = *number;
        }
    }

    max
}

fn get_char_max(char_list: &[char]) -> char {
    let mut max = char_list[0];

    for char in char_list {
        if char > &max {
            max = *char;
        }
    }

    max
}

fn main() {
    let num_list = vec![34, 56, 77, 25, 100, 54];
    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("num_list's max: {}", get_i32_max(&num_list));
    println!("char_list's max: {}", get_char_max(&char_list));
}