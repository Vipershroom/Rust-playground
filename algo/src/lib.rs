mod my_search;

#[cfg(test)]
mod tests {
    use crate::my_search;

    #[test]
    fn it_works() {
        let result: Vec<i32> = [1,2,3,4,5,6,7,8,].to_vec();
        println!("{}", my_search::binary_search(result, 8));
        
        // assert_eq!(result[3], 4);
    }
}
