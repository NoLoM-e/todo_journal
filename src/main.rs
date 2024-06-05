use std::path::PathBuf;

use cli::{Action, CliArgs};
use structopt::StructOpt;
use task::{add_task, complete_task, list_tasks, Task};

static DEFAULT_JOURNAL_PATH: &str = "journal.txt";

mod cli;
mod task;

fn main() {
    let app_args = CliArgs::from_args();
    let journal_path = match app_args.get_journal_path() {
        Some(expr) => expr,
        None => PathBuf::from(DEFAULT_JOURNAL_PATH),
    };
    let res = match app_args.get_action() {
        Action::Add { task } => add_task(journal_path, Task::new(task)),
        Action::Done { id } => complete_task(journal_path, id),
        Action::List => list_tasks(journal_path),
    };
    match res {
        Ok(()) => println!("Success"),
        Err(e) => panic!("Error was found {:?}", e),
    }
}
