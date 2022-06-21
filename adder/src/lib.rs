#[cfg(test)]


mod tests {
    #[test]
    fn exploration() {
        println!("I'm visible");
        assert_eq!(1,1)
    }

    #[test]
    #[ignore]
    fn mom() {
        println!("MOM")
    }
}
