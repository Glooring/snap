use crate::cli::{RemoteCreateArgs, SetupRepoArgs};
use crate::commands::remote;
use anyhow::Result;
use colored::*;

pub fn execute(args: SetupRepoArgs) -> Result<()> {
    println!(
        "{}",
        "[snap] setup-repo is a shortcut for `snap remote create`."
            .cyan()
            .bold()
    );

    remote::create_remote_repo(RemoteCreateArgs {
        repo: args.repo,
        private: args.private,
        public: args.public,
    })
}
