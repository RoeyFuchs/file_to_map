//! # File To Map
//! A crate that parses your file to a hashmap-base struct.
//! Can be used to parse configuration file easily.

use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::ops::Index;

const DEFAULT_KEY_VALUE_SEP: &str = "=";
const DEFAULT_PAIR_SEP: &str = "\n";

/// File To Map struct
pub struct FileToMap<'a> {
    key_value_sep: &'a str,
    pair_sep: &'a str,
    map: HashMap<String, String>,
    file_name: &'a str,
}

impl<'a> FileToMap<'a> {
    /// Creates a new struct. The file won't be parsed until calling the build function.
    pub fn new(file_name: &'a str) -> Self {
        FileToMap {
            key_value_sep: DEFAULT_KEY_VALUE_SEP,
            pair_sep: DEFAULT_PAIR_SEP,
            map: HashMap::new(),
            file_name: file_name,
        }
    }

    /// Changes the key-value pair separator. The default is `=`.
    pub fn set_key_value_separator(mut self, sep: &'a str) -> Self {
        self.key_value_sep = sep;
        return self;
    }

    /// Changes the pairs separator. The default is just a new line, but can be changed to any
    /// string.
    pub fn set_pair_separator(mut self, sep: &'a str) -> Self {
        self.pair_sep = sep;
        return self;
    }

    /// This function parses the file and creates the map. Call this function after changing the
    /// separators (if necessary). Be aware - this function returns a result struct. it will return an error due to an IO file error or missing key-value separator.
    pub fn build(mut self) -> Result<Self, Error> {
        let data = fs::read_to_string(self.file_name)?; // reading the file
        let data = self.strip_trailing_newline(&data); // remove the new-line in the end of the file

        let splitted_data: Vec<String> =
            data.split(self.pair_sep).map(|s| String::from(s)).collect(); // split by pairs separator

        let data_parse_result = splitted_data
            .iter()
            .try_for_each(|s| -> std::io::Result<()> {
                let key_val: Vec<&str> = s.splitn(2, self.key_value_sep).collect(); // split to key and value
                if key_val.len() != 2 {
                    // will happen when cannot find key-value separator
                    return Err(Error::new(
                        ErrorKind::Other,
                        format!("Cannot find {} in {}", self.key_value_sep, s),
                    ));
                }
                self.map
                    .insert(String::from(key_val[0]), String::from(key_val[1])); // add the pair to map
                Ok(())
            });

        let ret: Result<Self, Error> = match data_parse_result {
            Ok(()) => Ok(self),
            Err(e) => Err(e),
        };

        ret
    }

    /// Get value by key, same as get() of HashMap struct.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
    /// This function removes the trailing newline. Some of the text editors in unix-based systems adding this.
    fn strip_trailing_newline(&self, input: &'a str) -> &'a str {
        input
            .strip_suffix("\r\n")
            .or(input.strip_suffix("\n"))
            .unwrap_or(input)
    }
}

impl<'a> Index<&str> for FileToMap<'a> {
    type Output = String;
    /// Get value by key, using ["key"] (index operator).
    ///
    /// # Panic
    /// The function will panic when the key isn't in the map.
    fn index(&self, key: &str) -> &String {
        self.map
            .get(key)
            .expect(format!("Cannot find {} in FileToMap", key).as_str())
    }
}
#[cfg(test)]
mod test;
