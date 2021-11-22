use auto_domain_blocker::host_file;

const HOST_FILE_PATH: &str = "./fixtures/hosts.txt";

#[test]
fn test_host_file() {
    let f = host_file::HostFile::new(HOST_FILE_PATH).unwrap();

    assert!(f.cat().len() != 0);
    assert_eq!(f.which(), HOST_FILE_PATH);
}
