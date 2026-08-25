use anyhow::Result;
use clap::{Parser, Subcommand};
use remote::Source;
use std::path::PathBuf;

mod config;
mod init;
mod install;
mod manifest;
mod remote;
mod rewrite;

/// Invoked as `cargo nocli …`, so cargo hands the binary `nocli` as its first argument.
#[derive(Parser)]
#[command(bin_name = "cargo")]
enum Cargo {
    #[command(name = "nocli")]
    Nocli(Nocli),
}

#[derive(Parser)]
#[command(about = "Add nocomponents' styled layer to a Leptos project as source you own")]
struct Nocli {
    #[command(subcommand)]
    command: Command,

    /// Install from a nocomponents checkout rather than from the repository.
    #[arg(long, global = true, value_name = "DIR")]
    from: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Command {
    /// Write the config, install the stylesheet, and add the dependency.
    Init,
    /// Work with the styled layer.
    Components {
        #[command(subcommand)]
        command: Components,
    },
}

#[derive(Subcommand)]
enum Components {
    /// Install components, and whatever they are built out of.
    Add {
        names: Vec<String>,
        /// Overwrite components that are already installed.
        #[arg(long)]
        force: bool,
    },
    /// Show every component, marking the ones already installed.
    List,
}

#[tokio::main]
async fn main() {
    let Cargo::Nocli(cli) = Cargo::parse();

    if let Err(e) = run(cli).await {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

async fn run(cli: Nocli) -> Result<()> {
    let source = Source::new(cli.from)?;

    match cli.command {
        Command::Init => init::run(&source).await,
        Command::Components { command } => match command {
            Components::Add { names, .. } if names.is_empty() => {
                anyhow::bail!(
                    "name at least one component — `cargo nocli components list` shows them"
                )
            }
            Components::Add { names, force } => install::add(&source, &names, force).await,
            Components::List => install::list(&source).await,
        },
    }
}
