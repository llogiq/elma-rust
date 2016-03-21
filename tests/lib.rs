extern crate elma;
#[cfg(test)]
mod tests {
    use elma::{ lev, rec };
    use std::ffi::CString;

    #[test]
    fn test_lev_default_values () {
        let level = lev::Level::new();
        assert_eq!(level.version, "Elma".to_string());
        assert_eq!(level.link, 0);
        assert_eq!(level.integrity, [0.0_f64; 4]);
        assert_eq!(level.name, CString::new("").unwrap());
        assert_eq!(level.lgr, CString::new("default").unwrap());
        assert_eq!(level.ground, CString::new("ground").unwrap());
        assert_eq!(level.sky, CString::new("sky").unwrap());
    }

    #[test]
    fn test_lev_load_level () {
        let level = lev::Level::load_level("tests/test.lev");
        assert_eq!(level.version, "Elma".to_string());
        assert_eq!(level.link, 1524269776);
        assert_eq!(level.integrity, [-1148375.210607791,
                                      1164056.210607791,
                                      1162467.210607791,
                                      1162283.210607791]);
        assert_eq!(level.name, CString::new("Rust test").unwrap());
        assert_eq!(level.lgr, CString::new("default").unwrap());
        assert_eq!(level.ground, CString::new("ground").unwrap());
        assert_eq!(level.sky, CString::new("sky").unwrap());
        assert_eq!(level.polygons.len(), 2);
        assert_eq!(level.polygons, vec![lev::Polygon::new(), lev::Polygon::new()])
    }

    #[test]
    fn test_rec_default_values () {
        let rec = rec::Rec::new();
        assert_eq!(true, true);
    }
}
