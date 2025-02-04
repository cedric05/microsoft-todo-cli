// Copyright (c) Microsoft Corporation - 2022.
// Licensed under the MIT License.

mod cli;
mod tasks;

use clap::Parser;

use cli::{Cli, Commands::*};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Login {}) => tasks::login(),
        Some(Me { json }) => tasks::show_me(json),
        Some(Show { json }) => tasks::show_tasks(json),
        Some(Add { task }) => tasks::add_task(task),
        Some(Complete { id }) => tasks::complete_task(id),
        Some(Reopen { id }) => tasks::reopen_task(id),
        Some(Delete { id }) => tasks::delete_task(id),
        Some(Intr) => tasks::interactive(),
        None => {
            println!("Default subcommand");
            Ok(())
        }
    }?;

    Ok(())
}
