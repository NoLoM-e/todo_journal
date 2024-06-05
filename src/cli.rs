use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub enum Action {
    /// Add task to journal
    Add {
        /// Task description
        #[structopt()]
        task: String,
    },
    /// Mark task with number as Done and remove from journal
    Done {
        /// Posion of taks to remove
        #[structopt()]
        id: usize,
    },
    /// List all tasks in journal
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "My journal",
    about = "A command line tool to handle todo tasks in journals"
)]
pub struct CliArgs {
    #[structopt(subcommand)]
    action: Action,
    #[structopt(parse(from_os_str), short, long)]
    journal_path: Option<PathBuf>,
}

impl CliArgs {
    pub fn get_action(&self) -> Action {
        self.action.clone()
    }
    pub fn get_journal_path(&self) -> Option<PathBuf> {
        self.journal_path.clone()
    }
}
