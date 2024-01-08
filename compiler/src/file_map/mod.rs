mod char_codes;

use std::fs::File;
use std::io::{self, Read};

pub struct FileMap {
    data: Vec<u8>
}

impl FileMap {
    /// constructs a new FileMap object
    /// 
    /// the FileMap is used to read the source code
    /// 
    /// from a memory mapped file and load it into memory 
    /// 
    /// for lexical analysis
    pub fn new(file_path: &str) -> io::Result<Self> {
        let mut file: File = File::open(file_path)?;
        let file_size: usize = file.metadata()?.len() as usize;
        let mut data: Vec<_> = Vec::with_capacity(file_size);
        unsafe { data.set_len(file_size) }
        file.read_exact(&mut data)?;
        Ok(Self { data })
    }
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }
}