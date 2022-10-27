use anyhow::{bail, Context, Result};
use argh::FromArgs;
use internet_addiction_cessation::{config::Config, host_file::HostFile};
use chrono::prelude::*;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(FromArgs, Debug, PartialEq)]
/// This app help you get rid of internet addiction
struct Args {
    #[argh(subcommand)]
    nested: SubCommands,
}

#[derive(FromArgs, Debug, PartialEq)]
#[argh(subcommand)]
enum SubCommands {
    Block(CfgPathBlock),
    UnBlock(CfgPathUnBlock),
}

#[derive(FromArgs, Debug, PartialEq)]
#[argh(subcommand, name = "block")]
/// defining where the config is
struct CfgPathBlock {
    #[argh(short = 'p', option, default = "String::from(\"./domains.toml\")")]
    /// specify the config path
    path: String,
}

#[derive(FromArgs, Debug, PartialEq)]
#[argh(subcommand, name = "unblock")]
/// defining where the config is
struct CfgPathUnBlock {
    #[argh(short = 'p', option, default = "String::from(\"./domains.toml\")")]
    /// specify the config path
    path: String,
}

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .without_time()
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .with_context(|| "Fail to set tracing logger")?;

    let args: Args = argh::from_env();

    run(&args).with_context(|| "Fail to run block process")?;

    Ok(())
}

fn run(app: &Args) -> Result<()> {
    info!("Reading host file...");
    let mut hf = HostFile::new("/etc/hosts")?;

    match &app.nested {
        SubCommands::Block(path) => {
            let cfg_path = &path.path;
            info!("Reading config file {}", cfg_path);
            sudo::escalate_if_needed().unwrap();
            let cfg = Config::new(cfg_path)?;
            info!("Running block process...");
            hf.generate(&cfg)?;
            info!("URL block process success");
        }
        SubCommands::UnBlock(path) => {
            info!("Running unblock process...");
            sudo::escalate_if_needed().unwrap();
            let cfg_path = &path.path;
            info!("Reading config file {}", cfg_path);
            let cfg = Config::new(cfg_path)?;

            let can = can_unblock(&cfg).with_context(|| "Fail to unblock domains")?;

            if !can {
                error!("It is not the time for rest, you can't unblock those domains");
            } else {
                hf.recover()?;
                info!("Domains unblocked. Take a break, but don't overdo it");
            }
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
