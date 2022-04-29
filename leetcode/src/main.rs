

fn vec_to_target(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, elem) in nums.iter().enumerate() {
        for (k,elem2) in nums.iter().enumerate() {
            if elem + elem2 == target{
                return vec![i as i32,k as i32]
            }
        }
    }
    unreachable!()
}

fn main() {
    println!("{:?}", vec_to_target(vec![9,6,6,2], 6));
}
