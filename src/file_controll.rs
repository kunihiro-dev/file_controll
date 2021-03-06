pub mod reader {
    use std::io;
    use std::io::Read;
    use std::fs::File;

    pub fn read_file() -> Result<String, io::Error> {
        let f = File::open("test.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
}

