pub mod file{
    use std::fs::File;
    use std::io::{BufReader, BufRead};

    pub fn read_file_into_string(file_path: &'static str) -> Result<String, String> {
        let mut content = String::new();

        let file = File::open(file_path);
        let file = match file {
            Ok(f) => f,
            Err(e) => return Err(format!("Open file '{path}': {error}", path=file_path, error=e)),
        };

        let buffer = BufReader::new(file);
        for line in buffer.lines() {
            if let Ok(line) = line {
                content += &line;
            }
        }

        Ok(content)
    }
}
