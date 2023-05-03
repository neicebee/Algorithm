fn get_max<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];

    for &i in list.iter() {
        if i > max {
            max = i;
        }
    }

    max
}

fn main() {
    let num_list = vec![34, 56, 77, 25, 100, 54];
    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("max: {}", get_max(&num_list));
    println!("max: {}", get_max(&char_list));
}
