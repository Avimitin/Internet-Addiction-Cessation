use std::fs::File;
use std::io::prelude::*;

fn main() {
    let path = "hosts.txt";

    let file = File::create(path);
    let mut file = match file {
        Ok(f) => f,
        Err(e) => panic!("Fail to read {path}: {error}", path=path, error=e),
    };

    if let Err(e) = file.write_all(b"Hosts file\n") {
        panic!("Write file {}: {}", path, e)
    };
}
