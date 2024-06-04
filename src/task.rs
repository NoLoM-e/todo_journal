use chrono::Local;
use std::fmt;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::ErrorKind;
use std::path::PathBuf;

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    description: String,
    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(desc: String) -> Task {
        Task {
            description: desc,
            created_at: Utc::now(),
        }
    }
}

impl Display for Task {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let created_at_str = self.created_at.with_timezone(&Local).format("%F %H:%M");

        write!(fmt, "{:<30} [{}]", self.description, created_at_str)
    }
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<(), Error> {
    let mut tasks_from_journal = read_tasks_from_file(&journal_path)?;
    tasks_from_journal.push(task);

    write_tasks_to_file(&journal_path, tasks_from_journal);

    Ok(())
}

pub fn complete_task(journal_path: PathBuf, id: usize) -> Result<(), Error> {
    let mut tasks = read_tasks_from_file(&journal_path)?;

    if id == 0 || id > tasks.len() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "You don't have tusk numer {} in journal {}",
                id,
                journal_path.display()
            ),
        ));
    }
    tasks.remove(id - 1);

    write_tasks_to_file(&journal_path, tasks)?;

    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<(), Error> {
    let tasks = read_tasks_from_file(&journal_path)?;

    if tasks.is_empty() {
        println!("Task list is empty");
        return Ok(());
    }
    for (i, task) in tasks.iter().enumerate() {
        println!("{} - {}", i, task);
    }

    Ok(())
}

fn read_tasks_from_file(path: &PathBuf) -> Result<Vec<Task>, Error> {
    let journal = OpenOptions::new()
        .read(true)
        .create(true)
        .truncate(false)
        .open(path)?;

    let tasks_from_journal: Vec<Task> = match serde_json::from_reader(&journal) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };

    Ok(tasks_from_journal)
}

fn write_tasks_to_file(path: &PathBuf, tasks: Vec<Task>) -> Result<(), Error> {
    let journal = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    serde_json::to_writer(journal, &tasks)?;

    Ok(())
}
