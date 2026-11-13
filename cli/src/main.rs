use anyhow::Result;
use clap::{Parser, Subcommand};
use remote::Source;
use std::path::PathBuf;

mod config;
mod init;
mod install;
mod lock;
mod manifest;
mod modules;
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
    /// Work with the icons the components fall back on.
    Icons {
        #[command(subcommand)]
        command: Icons,
    },
}

#[derive(Subcommand)]
enum Icons {
    /// Install icons as source, and the `Icon` wrapper they are drawn with.
    Add {
        names: Vec<String>,
        /// Overwrite icons that are already installed, edited ones included.
        #[arg(long)]
        force: bool,
    },
    /// Show every icon, marking the ones already installed.
    List,
    /// Remove installed icons.
    Remove {
        names: Vec<String>,
        /// Remove even an icon that has been edited, or that something installed still uses.
        #[arg(long)]
        force: bool,
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
    /// Remove installed components.
    Remove {
        names: Vec<String>,
        /// Remove even a component that has been edited, or that another one is built on.
        #[arg(long)]
        force: bool,
    },
    /// Show every component, marking the ones already installed.
    List,
    /// Show which installed components and icons are edited, unchanged, or not in the lock.
    Status,
    /// Compare installed components and icons with the release in Cargo.lock.
    Update {
        /// Write the unchanged files that are outdated; edited ones are never touched.
        #[arg(long)]
        apply: bool,
    },
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
            Components::Remove { names, .. } if names.is_empty() => {
                anyhow::bail!("name at least one component to remove")
            }
            Components::Remove { names, force } => install::remove(&names, force),
            Components::List => install::list(&source).await,
            Components::Status => install::status(),
            Components::Update { apply } => install::update(&source, apply).await,
        },
        Command::Icons { command } => match command {
            Icons::Add { names, .. } if names.is_empty() => {
                anyhow::bail!("name at least one icon — `cargo nocli icons list` shows them")
            }
            Icons::Add { names, force } => install::add_icons(&source, &names, force).await,
            Icons::List => install::list_icons(&source).await,
            Icons::Remove { names, .. } if names.is_empty() => {
                anyhow::bail!("name at least one icon to remove")
            }
            Icons::Remove { names, force } => install::remove_icons(&names, force),
        },
    }
}
