use clap::{Parser, Subcommand};

use add::add_file;
use branch::create_branch;
use checkout::checkout;
use commit::commit_changes;
use init::init_repo;
use log::show_log;
use merge::merge_branch;

mod add;
mod branch;
mod checkout;
mod commit;
mod init;
mod log;
mod merge;

#[derive(Parser)]
#[command(name = "rustvcs")]
#[command(about = "A simple version control system written in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new repository
    Init,

    /// Add a file to the staging area
    Add {
        /// Path to the file
        file: String,
    },

    /// Commit changes to the repository
    Commit {
        /// Commit message
        message: String,
    },

    /// Show commit history
    Log,

    /// Checkout a branch or commit
    Checkout {
        /// Branch name or commit ID
        reference: String,
    },

    /// Create a new branch
    Branch {
        /// Name of the new branch
        name: String,
    },

    /// Merge a branch into the current branch
    Merge {
        /// Branch to merge
        branch: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_repo(),
        Commands::Add { file } => add_file(&file),
        Commands::Commit { message } => commit_changes(&message),
        Commands::Log => show_log(),
        Commands::Checkout { reference } => checkout(&reference),
        Commands::Branch { name } => create_branch(&name),
        Commands::Merge { branch } => merge_branch(&branch),
    }
}
