use clap::{App, ArgMatches, Arg};
use anyhow::{Context, Result};
use auto_domain_blocker::{config, host_file};

fn main() -> Result<()> {
    let app = build_cli_app();

    if let Some(debug_opt) = app.subcommand_matches("debug") {
        debug(debug_opt);
        return Ok(());
    }

    let path = app.value_of("config").unwrap_or("./domains.toml");
    let cfg = config::Config::new(path)?;

    run(&app, &cfg)
        .with_context(|| {
            format!("Run app with config: {} fail", path)
        })?;

    Ok(())
}

fn run(app: &ArgMatches, cfg: &config::Config) -> Result<()> {
    println!("Reading host file...");
    let mut hf = host_file::HostFile::new("/etc/hosts")?;

    if let Some(_) = app.subcommand_matches("block") {
        println!("Running block process");

        hf.generate(cfg)?;
        return Ok(());
    }

    if let Some(_) = app.subcommand_matches("unblock") {
        println!("Running unblock process");

        hf.remove()?;
    }

    Ok(())
}

fn debug(opt: &ArgMatches) {
    let path = opt.value_of("config").unwrap();
    println!("Reading config {}", path);

    let host = opt.value_of("host").unwrap();
    println!("Reading host file {}", host);

    println!("Creating config");
    let cfg = config::Config::new(path).unwrap();
    println!("Creating host file");
    let mut hf = host_file::HostFile::new(host).unwrap();

    println!("Testing config generate");
    hf.generate(&cfg).unwrap();

    println!("Testing config remove");
    hf.remove().unwrap();
}

fn build_cli_app() -> ArgMatches {
    App::new("Auto domains blocker")
        .version("0.1")
        .author("Avimitin <avimitin@gmail.com>")
        .about("This app help you get rid of internet addiction")
        .subcommands(vec![
            App::new("block")
                .about("Block all the domains now when it is time to study")
                    .arg(Arg::new("config")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG_PATH")
                        .about("Set the path to the user specific config file")
                        .takes_value(true)),
            App::new("unblock")
                .about("Unblock all the domains only when it is time to relax")
                    .arg(Arg::new("config")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG_PATH")
                        .about("Set the path to the user specific config file")
                        .takes_value(true)),
            App::new("debug")
                .about("Use this to debug program")
                .args(vec![
                    Arg::new("config")
                        .long("config")
                        .value_name("DEBUG_CONFIG"),
                    Arg::new("host")
                        .long("host")
                        .value_name("DEBUG_HOST_FILE"),
                ]),
        ])
        .get_matches()
}
