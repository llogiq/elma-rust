#[cfg(test)]
mod tests {
    extern crate elma;

    #[test]
    fn test_default_values_1 () {
        let level = elma::lev::Level::new();
        assert_eq!(level.link, 0);
    }

    #[test]
    #[should_panic]
    fn test_default_values_2 () {
        let level = elma::lev::Level::new();
        assert_eq!(level.link, 1);
    }
}
