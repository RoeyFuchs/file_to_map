[![Rust](https://github.com/RoeyFuchs/file_to_map/actions/workflows/rust.yml/badge.svg)](https://github.com/RoeyFuchs/file_to_map/actions/workflows/rust.yml)
# File To Map
A small parser for using data from a file as a map. 

This tool is best for programs that reading configuration in run-time but can be used for any data.
## Example

#### **`configuration.txt`**
```
width=1920
height=1080
```

#### **`code.rs`**
```
use file_to_map::FileToMap;
fn main() {
	let file_to_map = FileToMap::new("configuration.txt").build().unwrap();
	let width: i32 = file_to_map.get("width").unwrap().parse::<i32>().unwrap();
	let height: i32 = file_to_map["height"].parse::<i32>().unwrap();
	println!("{], {}", width, height);
}
```

The default separator between key and value is `=`, but it is possible to change it:

```
let file_to_map = FileToMap::new("configuration.txt").build().unwrap();
file_to_map.set_key_value_separator("*").build().unwrap();
```

The default separator between pairs is a new line, but it is possible to change it:

```
let file_to_map = FileToMap::new("configuration.txt").build().unwrap();
file_to_map.set_pair_separator(",").build().unwrap();
```

More examples can be found in the tests directory.

## License
MIT
