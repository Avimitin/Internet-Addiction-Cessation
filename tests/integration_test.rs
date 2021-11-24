use auto_domain_blocker::{config::Config, host_file::HostFile};
#[test]
fn test_all() {
    let path = "./fixtures/domains.toml";
    println!("Reading config {}", path);

    let host = "./fixtures/emptyhosts.txt";
    println!("Reading host file {}", host);
    let backup = std::fs::read_to_string(host).unwrap();

    println!("Creating config");
    let cfg = Config::new(path).unwrap();

    println!("Creating host file");
    let mut hf = HostFile::new(host).unwrap();

    println!("Testing config generate");
    hf.generate(&cfg).unwrap();

    println!("Generated contents: \n");
    println!("{}", hf.cat());

    println!("Written contents: \n");
    let file = std::fs::read_to_string(host).unwrap();
    println!("{}", file);

    println!("Testing config remove");
    hf.remove().unwrap();

    println!("Debug done");
    std::fs::write(host, backup).unwrap();
}
