use std::path::Path;

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use theseus::prelude::{State, process, profile};
use theseus::profile::QuickPlayType;

const APP_ID_CANDIDATES: &[&str] = &[
    "com.modrinth.ModrinthApp",
    "ModrinthApp",
    "com.modrinth.theseus",
];

#[derive(Debug, Parser)]
#[command(
    name = "mrlaunch",
    version,
    about = "Launch Modrinth instances from CLI"
)]
struct Cli {
    #[arg(long, env = "MRLAUNCH_APP_ID")]
    app_id: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    List,
    Run {
        profile_path: String,
        #[arg(long)]
        wait: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let app_id = resolve_app_id(cli.app_id)?;
    State::init(app_id.clone())
        .await
        .with_context(|| format!("failed to initialize Modrinth state for app id '{app_id}'"))?;

    match cli.command {
        Commands::List => list_profiles().await,
        Commands::Run { profile_path, wait } => run_profile(&profile_path, wait).await,
    }
}

async fn list_profiles() -> Result<()> {
    let mut profiles = profile::list().await.context("failed to fetch profiles")?;
    profiles.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    if profiles.is_empty() {
        println!("No profiles found.");
        return Ok(());
    }

    for p in profiles {
        println!("{}\t{}", p.path, p.name);
    }

    Ok(())
}

async fn run_profile(profile_path: &str, wait: bool) -> Result<()> {
    let launched = profile::run(profile_path, QuickPlayType::None)
        .await
        .with_context(|| format!("failed to run profile '{profile_path}'"))?;

    println!("uuid={}", launched.uuid);

    if wait {
        process::wait_for(launched.uuid)
            .await
            .with_context(|| format!("failed while waiting for process {}", launched.uuid))?;
    }

    Ok(())
}

fn resolve_app_id(explicit: Option<String>) -> Result<String> {
    if let Some(id) = explicit {
        return Ok(id);
    }

    let data_dir =
        dirs::data_dir().ok_or_else(|| anyhow!("unable to resolve OS data directory"))?;

    let mut matches: Vec<String> = APP_ID_CANDIDATES
        .iter()
        .copied()
        .filter(|id| has_existing_app_db(&data_dir, id))
        .map(ToOwned::to_owned)
        .collect();

    matches.sort();
    matches.dedup();

    match matches.as_slice() {
        [one] => Ok(one.clone()),
        [] => Err(anyhow!(
            "could not auto-detect Modrinth app id. Pass --app-id (or set MRLAUNCH_APP_ID). Tried: {}",
            APP_ID_CANDIDATES.join(", ")
        )),
        many => Err(anyhow!(
            "multiple possible app ids found ({}). Please pass --app-id explicitly",
            many.join(", ")
        )),
    }
}

fn has_existing_app_db(data_dir: &Path, app_id: &str) -> bool {
    data_dir.join(app_id).join("app.db").is_file()
}
