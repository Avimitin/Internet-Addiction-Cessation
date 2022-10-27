use internet_addiction_cessation::config::Config;
use internet_addiction_cessation::host_file;

const HOST_FILE_PATH: &str = "./fixtures/hosts.txt";
const CFG_FILE_PATH: &str = "./fixtures/domains.toml";
use std::fs;

#[test]
fn test_host_file() {
    let f = host_file::HostFile::new(HOST_FILE_PATH).unwrap();

    assert!(!f.contents.is_empty());
    assert_eq!(f.bound_index, Some((2, 11)));

    let keep = f.contents.clone();
    f.recover().unwrap();
    let current = fs::read_to_string(HOST_FILE_PATH).unwrap();
    assert_eq!(current, "127.0.0.1 localhost\n\n");
    // reset back
    fs::write(f.location, keep).unwrap();
}

#[test]
fn test_generate() {
    let path = "./fixtures/emptyhosts.txt";
    let backup = std::fs::read_to_string(path).unwrap();

    let mut f = host_file::HostFile::new(path).unwrap();
    let cfg = Config::new(CFG_FILE_PATH).unwrap();

    // Test generate
    f.generate(&cfg).unwrap();
    let contents = std::fs::read_to_string("./fixtures/emptyhosts.txt").unwrap();

    assert_eq!(contents.lines().count(), 7);

    std::fs::write(path, backup).unwrap();
}
