use std::fmt::Display;
use std::path::PathBuf;
use std::{fs, io, process};

use clap::{Parser, Subcommand};
use rand::seq::IndexedRandom;
use regex::Regex;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no configurations matching pattern '{regex}'")]
    NoMatch { regex: Regex },

    #[error("wg-quick failed with status code {code:?}")]
    WgQuick { code: Option<i32> },

    #[error("I/O error: {err}")]
    Io {
        #[from]
        err: io::Error,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Bring up a random configuration matching `REGEX`
    Up {
        /// Regex pattern to match against
        #[arg(value_name = "REGEX", default_value = "^")]
        pattern: Regex,
    },

    /// List configurations matching `REGEX`
    Ls {
        /// Regex pattern to match against
        #[arg(value_name = "REGEX", default_value = "^")]
        pattern: Regex,
    },

    /// Bring down currently running configuration
    Down,
}

#[derive(Parser, Debug, Clone)]
pub struct Cmdline {
    /// Directory WireGuard configurations are found in
    #[arg(short, long, value_name = "DIR", default_value = "/etc/wireguard")]
    dir: PathBuf,
    /// File to track currently running config
    #[arg(
        short,
        long,
        value_name = "FILE",
        default_value = "/run/wg-man.current"
    )]
    run_file: PathBuf,
    /// Whether to run `wg-quick`, or just write command to `stdout`
    #[arg(short, long, default_value_t = false)]
    mock: bool,
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // verbose: u8,
    #[command(subcommand)]
    command: Command,
}

fn ok_report<T, E: Display>(res: Result<T, E>, name: &str) -> Option<T> {
    match res {
        Ok(val) => Some(val),
        Err(err) => {
            log::warn!("{name}: {err}");
            None
        }
    }
}

fn read_configs(args: &Cmdline) -> Result<impl Iterator<Item = String>, Error> {
    let ents = fs::read_dir(&args.dir)?.filter_map(|e| {
        let ent = ok_report(e, "bad entry")?;
        let kind = ok_report(ent.file_type(), "file type")?;
        let name = ent.path().file_stem()?.to_str()?.to_string();

        if kind.is_file() || name.ends_with(".conf") {
            Some(name)
        } else {
            None
        }
    });

    Ok(ents)
}

fn get_matches(args: &Cmdline, regex: &Regex) -> Result<Vec<String>, Error> {
    let vec = read_configs(args)?.filter(|s| regex.is_match(s)).collect();
    Ok(vec)
}

fn bring_up(args: &Cmdline, conf: &str) -> Result<(), Error> {
    log::info!("bring config {conf} up");
    if args.mock {
        println!("wg-quick up {conf}");
        fs::write(&args.run_file, conf)?;

        Ok(())
    } else {
        let code = process::Command::new("wg-quick")
            .arg("up")
            .arg(conf)
            .spawn()?
            .wait()?;

        if code.success() {
            fs::write(&args.run_file, conf)?;
            Ok(())
        } else {
            Err(Error::WgQuick { code: code.code() }.into())
        }
    }
}

fn bring_down(args: &Cmdline) -> Result<Option<String>, Error> {
    if !args.run_file.exists() {
        return Ok(None);
    }

    let conf = fs::read_to_string(&args.run_file)?;
    log::info!("bring config {conf} down");

    if args.mock {
        println!("wg-quick down {conf}");

        Ok(Some(conf))
    } else {
        let code = process::Command::new("wg-quick")
            .arg("down")
            .arg(&conf)
            .spawn()?
            .wait()?;

        if code.success() {
            fs::remove_file(&args.run_file)?;

            Ok(Some(conf))
        } else {
            Err(Error::WgQuick { code: code.code() }.into())
        }
    }
}

struct FromFn<F>(F);

impl<F> FromFn<F> {
    pub fn new(op: F) -> FromFn<F>
    where
        F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
    {
        FromFn(op)
    }
}

impl<F> Display for FromFn<F>
where
    F: Fn(&mut std::fmt::Formatter<'_>) -> std::fmt::Result,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0(f)
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = Cmdline::parse();

    let mut rng = rand::rng();

    match &args.command {
        Command::Up { pattern } => {
            let ents = get_matches(&args, &pattern)?;

            log::debug!(
                "matches:\n{}",
                // nightly-only feature
                // fmt::from_fn(|f| {
                //     for s in ents.iter() {
                //         writeln!(f, "- {s}")?;
                //     }

                //     Ok(())
                // })
                FromFn::new(|f| {
                    for s in ents.iter() {
                        writeln!(f, "- {s}")?;
                    }

                    Ok(())
                })
            );

            let prev = bring_down(&args)?;
            let conf = loop {
                let choice = ents.choose(&mut rng);

                if ents.len() == 1 || ents.choose(&mut rng) != prev.as_ref() {
                    break choice;
                }
            };

            if let Some(conf) = conf {
                bring_up(&args, conf)?;
                Ok(())
            } else {
                Err(Error::NoMatch {
                    regex: pattern.clone(),
                }
                .into())
            }
        }
        Command::Ls { pattern } => {
            let ents = get_matches(&args, &pattern)?;

            println!(
                "matches:\n{}",
                //     fmt::from_fn(|f| {
                //         for s in ents.iter() {
                //             writeln!(f, "- {s}")?;
                //         }

                //         Ok(())
                //     })
                FromFn::new(|f| {
                    for s in ents.iter() {
                        writeln!(f, "- {s}")?;
                    }

                    Ok(())
                })
            );

            Ok(())
        }
        Command::Down => {
            bring_down(&args)?;
            Ok(())
        }
    }
}

// TODO: manage config files (add/remove from /etc/wireguard)
//       avoid wg-quick
//       write man page
//       make apk package
//
// to investigate:
//       daemon mode, will switch VPN every X amount of time
