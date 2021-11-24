use auto_domain_blocker::config::Config;

#[test]
fn test_read_config() {
    let cfg = Config::new("./fixtures/domains.toml").unwrap();

    assert_eq!(cfg.duration.start, "08:30");
    assert_eq!(cfg.duration.end, "21:30");
    assert_eq!(cfg.block_domains["bilibili.com"], vec!["www", "live", "@"]);
    assert_eq!(cfg.block_domains["youtube.com"], vec!["www"]);

    let block_domains = cfg.build_domains();
    println!("{:?}", block_domains);
    let expect_domains = ["www.bilibili.com", "bilibili.com", "live.bilibili.com", "www.youtube.com"];
    let mut match_count = 0;
    for domain in block_domains {
        for edomain in expect_domains {
            if domain == edomain {
                match_count += 1;
                break;
            }
        }
    }
    assert_eq!(match_count, 4);
}
