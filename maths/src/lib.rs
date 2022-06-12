mod sum;

#[cfg(test)]
mod tests {
    use crate::sum::my_sum;

    #[test]
    fn it_works() {
        let result = my_sum(2.0, 2.0);
        assert_eq!(result, 5.0);
    }
}
