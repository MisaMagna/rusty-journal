mod cli;
mod tasks;

use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn main() {
    // cli::CommandLineArgs::from_args();
    // println!("{:#?}", cli::CommandLineArgs::from_args());

    let CommandLineArgs {
        action,
        journal_file
    } = CommandLineArgs::from_args();
    
    let journal_file = journal_file.expect("Failed to find journal file");

    match action {
        Add { task } => tasks::add_task(journal_file, Task::new(task)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }
    .expect("Failed to perfom action")
}
