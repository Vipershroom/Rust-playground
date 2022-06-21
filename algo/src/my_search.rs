

pub fn binary_search<T: PartialOrd>(list: Vec<T>, target: T) -> i32 {
    let mut half: i32 = ((list.len() - 1) / 2).try_into().unwrap();
    loop {
        if list[half] == target {
            return half
        }
        else if list[half] > target {
            half += 1
        }
        else {
            half -= 1
        }
    }
}