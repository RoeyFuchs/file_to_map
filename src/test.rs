#[cfg(test)]
mod test{
    use crate::FileToMap;

    #[test]
    fn unchanged_defaults_get() {
        let file_to_map = FileToMap::new("tests_files/default.txt").build().unwrap();
        assert_eq!(file_to_map.get("width").unwrap(), "1920");
        assert_eq!(file_to_map.get("height").unwrap(), "1080");
    }
    #[test]
    fn unchanged_defaults_index_op() {
        let file_to_map = FileToMap::new("tests_files/default.txt").build().unwrap();
        assert_eq!(file_to_map["width"], "1920");
        assert_eq!(file_to_map["height"], "1080");
    }

    #[test]
    fn change_pair_sep() {
        let file_to_map = FileToMap::new("tests_files/pair_sep.txt").set_pair_separator("SEP").build().unwrap();
        assert_eq!(file_to_map["width"], "1920");
        assert_eq!(file_to_map["height"], "1080");
    }
    #[test]
    fn change_key_val_sep() {
        let file_to_map = FileToMap::new("tests_files/key_val_sep.txt").set_key_value_separator("SEP").build().unwrap();
        assert_eq!(file_to_map["width"], "1920");
        assert_eq!(file_to_map["height"], "1080");
    }
    #[test]
    fn change_separators() {
        let file_to_map = FileToMap::new("tests_files/separators_change.txt").set_pair_separator(":)").set_key_value_separator("SEP").build().unwrap();
        assert_eq!(file_to_map["width"], "1920");
        assert_eq!(file_to_map["height"], "1080");
        assert_eq!(file_to_map["hey"], "hoi");
    }
    #[test]
    fn file_doesnt_exist() {
        let file_to_map = FileToMap::new("42.txt").build();
        assert!(file_to_map.is_err());
    }

    #[test]
    fn key_doesnt_exist() {
        let file_to_map = FileToMap::new("tests_files/default.txt").build().unwrap();
        assert!(file_to_map.get("42").is_none());
    }
    #[test]
    #[should_panic]
    fn key_doesnt_exist_index_op() {
        let file_to_map = FileToMap::new("tests_files/default.txt").build().unwrap();
        let _ = file_to_map["42"];
    }

}



