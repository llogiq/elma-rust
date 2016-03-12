#[cfg(test)]
mod tests {
    extern crate elma;

    #[test]
    fn test_default_values () {
        let level = elma::lev::Level::new();
        assert_eq!(level.link, 0);
    }

    #[test]
    #[should_panic]
    fn test_default_values2 () {
        let level = elma::lev::Level::new();
        assert_eq!(level.link, 1);
    }
}
