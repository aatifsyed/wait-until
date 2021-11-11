use std::time::Duration;

use anyhow::Context;
use async_std::{
    process::{Command, Stdio},
    task::sleep,
};
use clap::AppSettings::TrailingVarArg;
use log::debug;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(setting = TrailingVarArg, about)]
struct Opt {
    /// Print the output of each run of cmd.
    #[structopt(short, long)]
    verbose: bool,

    /// How long to sleep between runs
    #[structopt(short, long, parse(try_from_str = parse_duration::parse))]
    wait: Option<Duration>,

    /// The command and arguments, repeatedly run until 0 is returned..
    /// The spawned process will receive a null stdin.
    /// stdout and stderr default to null, but will be inherited from the parent if --verbose is given.
    #[structopt(min_values = 1, required = true)]
    cmd: Vec<String>,
}

// Use `async` so that we can start adding timeouts etc easily in future.
#[async_std::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::try_init()?;
    let opt = Opt::from_args_safe()?;
    debug!("opt = {:?}", opt);

    loop {
        let mut cmd = Command::new(&opt.cmd[0]);
        let cmd = cmd
            .args(&opt.cmd[1..])
            .stdin(Stdio::null())
            .stdout(verbose_to_stdio(opt.verbose))
            .stderr(verbose_to_stdio(opt.verbose));
        debug!("cmd = {:?}", cmd);
        let status = cmd.status().await.context("Couldn't run command")?;
        debug!("status = {:?}", status);
        if status.success() {
            break;
        }
        if let Some(dur) = opt.wait {
            debug!("sleep for {:?}", dur);
            sleep(dur).await;
        }
    }
    Ok(())
}

fn verbose_to_stdio(verbose: bool) -> Stdio {
    match verbose {
        false => Stdio::null(),
        true => Stdio::inherit(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let opt = Opt::from_iter(&["wait-until", "ping", "-c", "1", "example.com"]);
        assert_eq!(opt.cmd, ["ping", "-c", "1", "example.com"]);
    }
}
