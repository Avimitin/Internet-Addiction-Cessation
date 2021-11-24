use anyhow::{bail, Context, Result};
use auto_domain_blocker::{config::Config, host_file::HostFile};
use chrono::prelude::*;
use clap::{App, Arg, ArgMatches};

fn main() -> Result<()> {
    let app = build_cli_app();

    let path = app.value_of("config").unwrap_or("./domains.toml");
    let cfg = Config::new(path)?;

    run(&app, &cfg).with_context(|| format!("Run app with config: {} fail", path))?;

    Ok(())
}

fn run(app: &ArgMatches, cfg: &Config) -> Result<()> {
    println!("Reading host file...");
    let mut hf = HostFile::new("/etc/hosts")?;

    if let Some(_) = app.subcommand_matches("block") {
        println!("Running block process");

        hf.generate(cfg)?;
        println!("URLs are all blocked");
        return Ok(());
    }

    if let Some(_) = app.subcommand_matches("unblock") {
        println!("Running unblock process");

        let can = can_unblock(cfg).with_context(|| format!("Fail to unblock domains"))?;

        if !can {
            println!("Focus on your work now!! It is not break time!!");
        } else {
            hf.remove()?;
            println!("Take a rest but don't too much~");
        }
    }

    Ok(())
}

fn can_unblock(cfg: &Config) -> Result<bool> {
    let local_now = Local::now();
    if let None = cfg.end_when() {
        bail!("Invalid time {}", cfg.duration.end);
    }
    let end_setting = cfg.end_when().unwrap();
    let end = Local::today().and_hms(end_setting.0, end_setting.1, 0);
    if local_now < end {
        return Ok(false);
    }

    Ok(true)
}

fn build_cli_app() -> ArgMatches {
    App::new("Auto domains blocker")
        .version("0.1")
        .author("Avimitin <avimitin@gmail.com>")
        .about("This app help you get rid of internet addiction")
        .subcommands(vec![
            App::new("block")
                .about("Block all the domains now when it is time to study")
                .arg(
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG_PATH")
                        .about("Set the path to the user specific config file")
                        .takes_value(true),
                ),
            App::new("unblock")
                .about("Unblock all the domains only when it is time to relax")
                .arg(
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG_PATH")
                        .about("Set the path to the user specific config file")
                        .takes_value(true),
                ),
        ])
        .get_matches()
}
