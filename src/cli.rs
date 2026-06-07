use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "A Git-powered local checkpoint workflow tool."
)]
#[command(
    long_about = "snap is a friendly Git-powered workflow tool. It manages versioned snapshots and also wraps common Git/GitHub workflows with safer, clearer commands."
)]
#[command(after_help = "\
Workflow groups:
  Daily workflow:    snap status | snap save \"message\" | snap history | snap update-repo \"message\" | snap sync
  Snapshots:         snap new <label> \"description\" | snap list --all-branches | snap list --branch main
  History:           snap history | snap history --all-branches
  Branches:          snap branch list | snap branch new feature-x | snap branch switch main
  Remote/GitHub:     snap remote status | snap setup-repo owner/repo --private | snap delete-repo owner/repo
  Release:           snap release windows | snap release upload | snap release list
  Diagnostics:       snap doctor | snap doctor --json --ci | snap doctor --repair | snap options
  Learn by example:  snap examples
")]
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
    /// Show a friendly Git and snap status summary
    Status(StatusArgs),
    /// Stage all changes and create a normal Git commit
    Save(SaveArgs),
    /// Push the current branch, snapshot tags, and snap metadata refs
    Push(PushArgs),
    /// Pull branch updates, snapshot tags, and snap metadata refs
    Pull(PullArgs),
    /// Pull and push branch updates, snapshot tags, and snap metadata refs
    Sync(SyncArgs),
    /// Show recent Git history with branches and snapshot tags
    History(HistoryArgs),
    /// Work with local Git branches through safer snap commands
    Branch(BranchArgs),
    /// Manage Git remotes and GitHub repositories
    Remote(RemoteArgs),
    /// Build GitHub release assets using the bundled release scripts
    Release(ReleaseArgs),
    /// Shortcut: save changes, then run snap-aware sync
    #[command(name = "update-repo")]
    UpdateRepo(UpdateRepoArgs),
    /// Shortcut: create a GitHub repo and connect it as origin
    #[command(name = "setup-repo")]
    SetupRepo(SetupRepoArgs),
    /// Shortcut: make a GitHub repository public after strict confirmation
    #[command(name = "make-public")]
    MakePublic(VisibilityRepoArgs),
    /// Shortcut: make a GitHub repository private after confirmation
    #[command(name = "make-private")]
    MakePrivate(VisibilityRepoArgs),
    /// Shortcut: delete a GitHub repository after strict confirmation
    #[command(name = "delete-repo")]
    DeleteRepo(DeleteRepoArgs),
    /// Show practical workflows for common snap commands
    Examples(ExamplesArgs),
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
    /// Diagnose Git repository health without making changes
    Doctor(DoctorArgs),
    /// Configure global snap options
    Options(OptionsArgs),
}

#[derive(Args, Debug)]
pub struct InitArgs {}

#[derive(Args, Debug)]
pub struct NewArgs {
    /// The short, memorable name for the snapshot (e.g., "v1.0")
    pub label: String,
    /// Create a snapshot even when only snap metadata changed
    #[arg(long)]
    pub include_metadata_only: bool,
    /// A longer description of the changes in this snapshot
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub description: Vec<String>,
}

#[derive(Args, Debug)]
#[command(after_help = "\
Branch filters use Git reachability: a snapshot belongs to a branch when the snapshot commit is reachable from that local branch.

Branch column values:
  <branch>            reachable from one local branch
  <branch> (shared)   reachable from multiple branches, including the current branch
  shared              reachable from multiple branches, not including the current branch
  unattached          snapshot tag exists, but no local branch reaches it

Examples:
  snap list
  snap list 10
  snap list --branch main
  snap list 1 --branch feature-login
  snap list --all-branches
")]
pub struct ListArgs {
    /// Number of snapshots to show, or "all". Overrides the configured default.
    pub limit: Option<String>,
    /// Show only snapshots reachable from this local branch.
    #[arg(long, conflicts_with = "all_branches")]
    pub branch: Option<String>,
    /// Show snapshots from all local branches with a Branch column.
    #[arg(long, conflicts_with = "branch")]
    pub all_branches: bool,
}

#[derive(Args, Debug)]
pub struct StatusArgs {}

#[derive(Args, Debug)]
pub struct SaveArgs {
    /// Commit message. If omitted, snap will prompt for it.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub message: Vec<String>,
}

#[derive(Args, Debug)]
pub struct PushArgs {}

#[derive(Args, Debug)]
pub struct PullArgs {}

#[derive(Args, Debug)]
pub struct SyncArgs {}

#[derive(Args, Debug)]
#[command(after_help = "\
History is read-only. It shows Git decorations, so branches and snapshot tags are visible next to commits.

By default, history shows the current branch/HEAD. Use --branch to inspect another local branch, or --all-branches to see a graph across local branches and snapshot tags.

Examples:
  snap history
  snap history 50
  snap history all
  snap history --branch main
  snap history --all-branches
")]
pub struct HistoryArgs {
    /// Number of commits to show, or "all". Defaults to 20.
    pub limit: Option<String>,
    /// Show history for this local branch.
    #[arg(long, conflicts_with = "all_branches")]
    pub branch: Option<String>,
    /// Show history across all local branches and snapshot tags.
    #[arg(long, conflicts_with = "branch")]
    pub all_branches: bool,
}

#[derive(Args, Debug)]
#[command(after_help = "\
Branch commands are safe wrappers around local Git branch operations.
Use `snap sync` after creating or switching branches when you want to publish branch, snapshot tags, and snap metadata.

Examples:
  snap branch list
  snap branch new feature-login
  snap branch switch main
  snap branch delete feature-login
  snap branch delete old-experiment --force
  snap branch merge feature-login
")]
pub struct BranchArgs {
    #[command(subcommand)]
    pub command: BranchCommands,
}

#[derive(Subcommand, Debug)]
pub enum BranchCommands {
    /// List local branches and their upstream status
    List(BranchListArgs),
    /// Create a new local branch and switch to it
    New(BranchNameArgs),
    /// Switch to an existing local branch
    Switch(BranchNameArgs),
    /// Delete a local branch
    Delete(BranchDeleteArgs),
    /// Merge a local branch into the current branch
    Merge(BranchNameArgs),
}

#[derive(Args, Debug)]
pub struct BranchListArgs {}

#[derive(Args, Debug)]
pub struct BranchNameArgs {
    /// Local branch name
    pub name: String,
}

#[derive(Args, Debug)]
pub struct BranchDeleteArgs {
    /// Local branch name
    pub name: String,
    /// Force delete an unmerged branch after exact branch-name confirmation
    #[arg(long)]
    pub force: bool,
}

#[derive(Args, Debug)]
#[command(after_help = "\
GitHub remote commands use the GitHub CLI for authentication.
Run `gh auth login` once, then retry the snap command.

Examples:
  snap remote status
  snap remote create owner/repo --private
  snap remote create owner/repo --public
  snap remote visibility public owner/repo
  snap remote visibility private owner/repo
  snap remote delete owner/repo
  snap remote set-url https://github.com/owner/repo.git
")]
pub struct RemoteArgs {
    #[command(subcommand)]
    pub command: RemoteCommands,
}

#[derive(Subcommand, Debug)]
pub enum RemoteCommands {
    /// Show Git remote and GitHub authentication status
    Status(RemoteStatusArgs),
    /// Create a GitHub repository and connect it as origin
    Create(RemoteCreateArgs),
    /// Change GitHub repository visibility
    Visibility(RemoteVisibilityArgs),
    /// Delete a GitHub repository after typing owner/repo exactly
    Delete(RemoteDeleteArgs),
    /// Add origin for an existing remote repository
    SetUrl(RemoteSetUrlArgs),
}

#[derive(Args, Debug)]
pub struct RemoteStatusArgs {}

#[derive(Args, Debug)]
pub struct RemoteCreateArgs {
    /// Repository in owner/name format
    pub repo: String,
    /// Create a private GitHub repository. This is the default.
    #[arg(long, conflicts_with = "public")]
    pub private: bool,
    /// Create a public GitHub repository.
    #[arg(long, conflicts_with = "private")]
    pub public: bool,
}

#[derive(Args, Debug)]
pub struct RemoteSetUrlArgs {
    /// Remote URL to add as origin
    pub url: String,
}

#[derive(Args, Debug)]
pub struct RemoteVisibilityArgs {
    #[command(subcommand)]
    pub command: RemoteVisibilityCommands,
}

#[derive(Subcommand, Debug)]
pub enum RemoteVisibilityCommands {
    /// Make a GitHub repository public after typing owner/repo exactly
    Public(VisibilityRepoArgs),
    /// Make a GitHub repository private after explicit confirmation
    Private(VisibilityRepoArgs),
}

#[derive(Args, Debug)]
pub struct VisibilityRepoArgs {
    /// Repository in owner/name format
    pub repo: String,
}

#[derive(Args, Debug)]
pub struct RemoteDeleteArgs {
    /// Repository in owner/name format
    pub repo: String,
}

#[derive(Args, Debug)]
pub struct DeleteRepoArgs {
    /// Repository in owner/name format
    pub repo: String,
}

#[derive(Args, Debug)]
#[command(after_help = "\
Release commands call the existing scripts in scripts/ and print the expected GitHub Release assets.
They do not create a GitHub Release and do not commit generated artifacts.

Examples:
  snap release windows
  snap release linux
  snap release all
  snap release upload
  snap release list
  snap release upload --publish
")]
pub struct ReleaseArgs {
    #[command(subcommand)]
    pub command: ReleaseCommands,
}

#[derive(Subcommand, Debug)]
pub enum ReleaseCommands {
    /// Run scripts/release-windows.ps1
    Windows(ReleasePlatformArgs),
    /// Run scripts/release-linux.sh
    Linux(ReleasePlatformArgs),
    /// Run both local release scripts after checking both runtimes
    All(ReleasePlatformArgs),
    /// Upload release-github/vX.Y.Z assets to a GitHub Release
    Upload(ReleaseUploadArgs),
    /// List recent GitHub Releases
    List(ReleaseListArgs),
}

#[derive(Args, Debug)]
pub struct ReleasePlatformArgs {}

#[derive(Args, Debug)]
pub struct ReleaseUploadArgs {
    /// GitHub repository in owner/repo format. Defaults to GitHub origin.
    #[arg(long)]
    pub repo: Option<String>,
    /// Publish the release immediately instead of creating a draft.
    #[arg(long)]
    pub publish: bool,
    /// Upload to an existing release and overwrite assets with the same names.
    #[arg(long)]
    pub clobber: bool,
}

#[derive(Args, Debug)]
#[command(after_help = "\
Release list is read-only. It uses GitHub CLI and defaults to the GitHub repository configured as origin.

Examples:
  snap release list
  snap release list 20
  snap release list --repo owner/repo
")]
pub struct ReleaseListArgs {
    /// Number of GitHub Releases to show. Defaults to 10.
    pub limit: Option<String>,
    /// GitHub repository in owner/repo format. Defaults to GitHub origin.
    #[arg(long)]
    pub repo: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpdateRepoArgs {
    /// Commit message for the save step. If omitted, snap will prompt for it.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub message: Vec<String>,
}

#[derive(Args, Debug)]
pub struct SetupRepoArgs {
    /// Repository in owner/name format
    pub repo: String,
    /// Create a private GitHub repository. This is the default.
    #[arg(long, conflicts_with = "public")]
    pub private: bool,
    /// Create a public GitHub repository.
    #[arg(long, conflicts_with = "private")]
    pub public: bool,
}

#[derive(Args, Debug)]
pub struct ExamplesArgs {}

#[derive(Args, Debug)]
pub struct RestoreArgs {
    /// The ID or label of the snapshot to restore. Shows a menu if omitted.
    pub id_or_label: Option<String>,
    /// Preview the restore target and rescue behavior without changing files
    #[arg(long)]
    pub dry_run: bool,
    /// Skip the rescue snapshot created before restore
    #[arg(long)]
    pub no_rescue: bool,
}

#[derive(Args, Debug)]
pub struct DeleteArgs {
    /// The ID or label of the snapshot to delete. Shows a menu if omitted.
    pub id_or_label: Option<String>,
    /// Also prune Git objects reachable only from this snapshot
    #[arg(long)]
    pub purge: bool,
    /// Skip the default purge bundle backup
    #[arg(long)]
    pub no_backup: bool,
}

#[derive(Args, Debug)]
pub struct EditArgs {
    /// The ID or label of the snapshot to edit. Shows a menu if omitted.
    pub id_or_label: Option<String>,
}

#[derive(Args, Debug)]
pub struct UpdateArgs {
    /// Update the active snapshot even when only snap metadata changed
    #[arg(long)]
    pub include_metadata_only: bool,
}

#[derive(Args, Debug)]
pub struct DiffArgs {
    /// The first snapshot (ID or label) to compare
    pub snapshot_a: String,
    /// The second snapshot (ID or label) to compare
    pub snapshot_b: String,
}

#[derive(Args, Debug)]
pub struct DoctorArgs {
    /// Repair safe Git corruption cases after creating a .git backup
    #[arg(long, conflicts_with_all = ["json", "ci"])]
    pub repair: bool,
    /// Rewrite historical snapshot tags to forget missing/invalid metadata refs
    #[arg(long, requires = "repair")]
    pub accept_metadata_loss: bool,
    /// Print machine-readable JSON instead of the human report
    #[arg(long)]
    pub json: bool,
    /// Exit non-zero when warnings or errors are found
    #[arg(long)]
    pub ci: bool,
}

#[derive(Args, Debug)]
pub struct OptionsArgs {}
