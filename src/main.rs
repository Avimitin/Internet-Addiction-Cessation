use anyhow::{bail, Context, Result};
use auto_domain_blocker::{config::Config, host_file::HostFile};
use chrono::prelude::*;
use clap::{App, Arg, ArgMatches};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .with_context(|| "Fail to set tracing logger")?;

    let app = build_cli_app();

    run(&app).with_context(|| "Fail to run block process")?;

    Ok(())
}

fn run(app: &ArgMatches) -> Result<()> {
    info!("Reading host file...");
    let mut hf = HostFile::new("/etc/hosts")?;

    if let Some(b_opt) = app.subcommand_matches("block") {
        let path = b_opt.value_of("config").unwrap_or("./domains.toml");
        info!("Reading config file {}", path);
        let cfg = Config::new(path)?;

        info!("Running block process...");

        hf.generate(&cfg)?;
        info!("URL block process success");
        return Ok(());
    }

    if let Some(ub_opt) = app.subcommand_matches("unblock") {
        info!("Running unblock process...");
        let path = ub_opt.value_of("config").unwrap_or("./domains.toml");
        info!("Reading config file {}", path);
        let cfg = Config::new(path)?;

        let can = can_unblock(&cfg).with_context(|| "Fail to unblock domains")?;

        if !can {
            println!("===============================================");
            println!("Focus on your work now!! It is not break time!!");
            println!("===============================================");
        } else {
            hf.remove()?;
            println!("===============================================");
            println!("Take a rest but don't too much~");
            println!("===============================================");
        }
    }

    Ok(())
}

fn can_unblock(cfg: &Config) -> Result<bool> {
    let local_now = Local::now();
    if cfg.end_when().is_none() {
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
