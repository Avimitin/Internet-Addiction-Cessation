use auto_domain_blocker::file::*;

const HOST_FILE_PATH: &str = "./fixtures/hosts.txt";
const HOST_FILE_CONTENT: &str = "0.0.0.0 bilibili.com\n";

#[test]
fn test_host_file() {
    let f = host_file::HostFile::new(HOST_FILE_PATH).unwrap();

    assert_eq!(f.cat(), HOST_FILE_CONTENT);
    assert_eq!(f.which(), HOST_FILE_PATH);
}
