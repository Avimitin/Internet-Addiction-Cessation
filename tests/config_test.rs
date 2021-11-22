use auto_domain_blocker::config::Config;

#[test]
fn test_read_config() {
    let cfg = Config::new("./fixtures/domains.toml");

    assert_eq!(cfg.duration.start, "08:30");
    assert_eq!(cfg.duration.end, "21:30");
    assert_eq!(cfg.block_domains["bilibili.com"], vec!["www", "live", "@"]);
    assert_eq!(cfg.block_domains["youtube.com"], vec!["www"]);

    let block_domains = cfg.build_domains();
    assert!(block_domains.contains("www.bilibili.com"));
    assert!(block_domains.contains("bilibili.com"));
    assert!(block_domains.contains("live.bilibili.com"));
    assert!(block_domains.contains("www.youtube.com"));
}
