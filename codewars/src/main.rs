pub fn two_sum(nums: Vec<i32>, target: i32) {
    let mut new_vec: Vec<i32> = Vec::new();
    let mut end = false;
    let mut first_indice: i32 = 0;
    let mut second_indice: i32 = 0;
    for (i,elem) in nums.iter().enumerate() {
        for (k, elem2) in nums.iter().enumerate() {
            if elem + elem2 == target && i != k {
                first_indice = i as i32;
                second_indice = k as i32;
                end = true;
                break;
            }
        }
        if end {
            break;
        }
    }
    new_vec.push(first_indice);
    new_vec.push(second_indice);
    println!("{:?}", new_vec)
}
fn main() {
    two_sum([3,2].to_vec(), 6)
}
