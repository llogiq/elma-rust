extern crate elma;
#[cfg(test)]
mod tests {
    use elma::{ lev, rec };

    #[test]
    fn test_lev_default_values_1 () {
        let level = lev::Level::new();
        assert_eq!(level.link, 0);
    }

    #[test]
    fn test_lev_load_level_1 () {
        let level = lev::Level::load_level("tests/test.lev");
        assert_eq!(level.version, "Elma".to_string());
        assert_eq!(level.link, 457242863);
    }

    #[test]
    fn test_rec_default_values_1 () {
        let rec = rec::Rec::new();
        assert_eq!(true, true);
    }
}
