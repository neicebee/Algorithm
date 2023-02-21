mod changer;
use crate::changer::operations;

fn main() {
    let mut nums: Vec<i32> = operations::make_integers();
    nums.sort();
    let avg = operations::get_avg(&nums);
    let mode = operations::get_mode(&nums);

    print!("평균값 : {avg}\n중간값 : {}\n최빈값 : ", nums[nums.len()/2]);
    
    for i in &mode {
        print!("{i} ");
    }
}