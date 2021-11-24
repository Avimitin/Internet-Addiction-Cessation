use auto_domain_blocker::host_file;
use auto_domain_blocker::config::Config;

const HOST_FILE_PATH: &str = "./fixtures/hosts.txt";
const CFG_FILE_PATH: &str = "./fixtures/domains.toml";

#[test]
fn test_host_file() {
    let f = host_file::HostFile::new(HOST_FILE_PATH).unwrap();

    assert!(f.cat().len() != 0);
    assert_eq!(f.which(), HOST_FILE_PATH);
    assert_eq!(f.read_bound_index(), Some((3, 12)));

    let keep = f.cat().clone();
    f.remove().unwrap();
    // reset back
    std::fs::write(f.which(), keep).unwrap();
}

#[test]
fn test_generate() {
    let mut f = host_file::HostFile::new("./fixtures/emptyhosts.txt").unwrap();
    let cfg = Config::new(CFG_FILE_PATH).unwrap();
    f.generate(&cfg).unwrap();
    let contents = std::fs::read_to_string("./fixtures/emptyhosts.txt").unwrap();
    assert_eq!(contents, "127.0.0.1 localhost

## <!-- auto domain blocker -->
0.0.0.0 www.bilibili.com
0.0.0.0 live.bilibili.com
0.0.0.0 bilibili.com
0.0.0.0 www.youtube.com
## <!-- auto domain blocker -->
")
}
