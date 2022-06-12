pub fn my_sum<T: std::ops::Add<Output = T>> (a: T,b: T) -> T {
    a + b
}