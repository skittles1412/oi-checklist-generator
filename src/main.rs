use crate::{
    online_judges::{dmoj, ojuz},
    theme::{DmojProgress, OjuzProgress, Theme},
};
use anyhow::Context;
use cache::Cache;
use clap::Parser;
use directories::{ProjectDirs, UserDirs};
use reqwest::cookie;
use std::{
    fs::{self, File},
    io::BufReader,
    path::{Path, PathBuf},
};
use tokio::sync::mpsc;

mod cache;
mod online_judges;
mod render;
mod theme;

fn load_cache_from_file(theme: &Theme, file: impl AsRef<Path>) -> Cache {
    if let Ok(file) = File::open(file) {
        if let Ok(cache) = serde_json::from_reader(BufReader::new(file)) {
            return cache;
        }
    }

    theme.log_warn("failed to load cache from file (either it didn't exist or it is corrupted)");
    theme.log_info("using fresh cache");

    Default::default()
}

fn save_cache_to_file(t: &Cache, file: impl AsRef<Path>) -> anyhow::Result<()> {
    serde_json::to_writer(
        File::create(file).context("failed to open file to save cache to")?,
        t,
    )
    .expect("failed to serialize cache");

    Ok(())
}

// TODO: codeforces support
#[derive(Parser)]
struct Cli {
    /// DMOJ username
    #[arg(long, alias("dmoj"))]
    dmoj_username: Option<String>,

    /// oj.uz username
    #[arg(long, alias("ojuz"))]
    ojuz_username: Option<String>,

    /// output location
    #[arg(short, long)]
    output: Option<String>,

    /// Open the output file in the default web browser
    #[arg(long, action)]
    open: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut theme = Theme::default();

    let project_dirs = ProjectDirs::from("", "", "OI Checklist Generator")
        .context("failed to find project directory location")?;

    let ojs = {
        let cache_location = {
            let cache_dir = project_dirs.cache_dir();
            fs::create_dir_all(cache_dir).context("failed to create cache directory")?;
            cache_dir.join("cache.json")
        };

        let mut cache = load_cache_from_file(&theme, &cache_location);

        let (dmoj_config, dmoj_progress) = match cli.dmoj_username {
            Some(username) => {
                let (problems_tx, problems_rx) = mpsc::unbounded_channel();
                let (submissions_tx, submissions_rx) = mpsc::unbounded_channel();

                (
                    Some(dmoj::Config {
                        username,
                        problems_channel: problems_tx,
                        submissions_channel: submissions_tx,
                    }),
                    Some(DmojProgress {
                        problems_channel: problems_rx,
                        submissions_channel: submissions_rx,
                    }),
                )
            }
            None => (None, None),
        };

        let (ojuz_config, ojuz_progress) = match cli.ojuz_username {
            Some(username) => {
                let (tx, rx) = mpsc::unbounded_channel();

                (
                    Some(ojuz::Config {
                        username,
                        channel: tx,
                    }),
                    Some(OjuzProgress { channel: rx }),
                )
            }
            None => (None, None),
        };

        let progress = theme.display_parser_progress(dmoj_progress, ojuz_progress);

        let parse_result = cache
            .parse(dmoj_config, ojuz_config, None, cookie::Jar::default())
            .await
            .context("parsing submissions");
        drop(progress);

        let ojs = match parse_result {
            Ok(ojs) => ojs,
            Err(e) => {
                theme.log_error(
                    "Oh noes, something happened when parsing your submissions (details below)",
                );
                theme.log_error("Maybe try ignoring the faulty online judge and try again");
                theme.println("");

                return Err(e);
            }
        };

        if let Err(e) = save_cache_to_file(&cache, cache_location).context("failed to save cache") {
            theme.log_warn("failed to save cache (details below)");
            theme.println("");

            theme.println(format!("{e:?}"));
        }

        ojs
    };

    let output_html = render::render_checklist(ojs).await;

    fn default_output_location() -> anyhow::Result<PathBuf> {
        Ok(UserDirs::new()
            .context("failed to find user directory location")?
            .document_dir()
            .context("failed to find user document directory location")?
            .join("oi-checklist.html"))
    }

    let output_location = match cli.output {
        Some(o) => o.into(),
        None => default_output_location()
            .context("failed to find the default user directory location")?,
    };

    fs::write(&output_location, output_html).context("failed to write to the output file")?;

    theme.log_info(format!(
        "Done! Output file can be found at {}",
        fs::canonicalize(&output_location)
            .unwrap_or(output_location.clone())
            .display()
    ));

    if cli.open {
        if let Err(e) = open::that(&output_location) {
            theme.log_error("failed to open file in the default web browser");
            theme.println("");

            theme.println(format!("{e:?}"));
        }
    }

    Ok(())
}
