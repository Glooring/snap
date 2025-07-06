use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "A blazing fast, Git-powered snapshot tool.", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new snap repository for this project
    Init(InitArgs),
    /// Create a new versioned snapshot of the project
    New(NewArgs),
    /// List all available snapshots for this project
    List(ListArgs),
    /// Restore project to a previous snapshot state
    Restore(RestoreArgs),
    /// Permanently delete a snapshot and its data
    Delete(DeleteArgs),
    /// Edit the label and description of a snapshot
    Edit(EditArgs),
    /// Replaces the latest snapshot with the current project state
    Update(UpdateArgs),
    /// Compare two snapshots and show a list of changes
    Diff(DiffArgs),
    /// Configure global snap options
    Options(OptionsArgs),
}

#[derive(Args, Debug)]
pub struct InitArgs {}

#[derive(Args, Debug)]
pub struct NewArgs {
    /// The short, memorable name for the snapshot (e.g., "v1.0")
    pub label: String,
    /// A longer description of the changes in this snapshot
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub description: Vec<String>,
}

#[derive(Args, Debug)]
pub struct ListArgs {}

#[derive(Args, Debug)]
pub struct RestoreArgs {
    /// The ID or label of the snapshot to restore. Shows a menu if omitted.
    pub id_or_label: Option<String>,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// The ID or label of the snapshot to delete. Shows a menu if omitted.
    pub id_or_label: Option<String>,
}

#[derive(Args, Debug)]
pub struct EditArgs {
    /// The ID or label of the snapshot to edit. Shows a menu if omitted.
    pub id_or_label: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpdateArgs {}

#[derive(Args, Debug)]
pub struct DiffArgs {
    /// The first snapshot (ID or label) to compare
    pub snapshot_a: String,
    /// The second snapshot (ID or label) to compare
    pub snapshot_b: String,
}

#[derive(Args, Debug)]
pub struct OptionsArgs {}