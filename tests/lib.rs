extern crate elma;
#[cfg(test)]
mod tests {
    use elma::{ lev, rec };
    use std::ffi::CString;

    #[test]
    fn test_lev_default_values_1 () {
        let level = lev::Level::new();
        assert_eq!(level.link, 0);
    }

    #[test]
    fn test_lev_load_level_1 () {
        let level = lev::Level::load_level("tests/test.lev");
        assert_eq!(level.version, "Elma".to_string());
        assert_eq!(level.link, 1524269776);
        assert_eq!(level.integrity, [-1148375.210607791,
                                      1164056.210607791,
                                      1162467.210607791,
                                      1162283.210607791]);
        assert_eq!(level.name, CString::new("Rust test").unwrap());
    }

    #[test]
    fn test_rec_default_values_1 () {
        let rec = rec::Rec::new();
        assert_eq!(true, true);
    }
}
