mod file_map;
use file_map::FileMap;

fn main() {
    let fm: Result<FileMap, std::io::Error> = FileMap::new("C:/Users/ellio/OneDrive/Documents/GitHub/Compiler-in-Rust/compiler/src/hello.txt");
    let file_map: FileMap = fm.expect("Failed to create File Map");
    println!("File contents: {:?}", file_map.as_slice());
}