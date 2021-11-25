use auto_domain_blocker::{config::Config, host_file::HostFile};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[test]
fn test_all() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    let path = "./fixtures/domains.toml";
    info!("Reading config {}", path);

    let host = "./fixtures/emptyhosts.txt";
    info!("Reading host file {}", host);
    let backup = std::fs::read_to_string(host).unwrap();

    info!("Creating config");
    let cfg = Config::new(path).unwrap();

    info!("Creating host file");
    let mut hf = HostFile::new(host).unwrap();

    info!("Testing config generate");
    hf.generate(&cfg).unwrap();

    info!("Generated contents: \n");
    println!("{}", hf.cat());

    info!("Written contents: \n");
    let file = std::fs::read_to_string(host).unwrap();
    println!("{}", file);

    info!("Testing config remove");
    hf.remove().unwrap();

    info!("Debug done");
    std::fs::write(host, backup).unwrap();
}
