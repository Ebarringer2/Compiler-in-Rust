mod file_map;
use file_map::FileMap;

fn main() {
    let file_map = FileMap::new("./hello.txt");
    println!("{}", file_map.as_slice())
}