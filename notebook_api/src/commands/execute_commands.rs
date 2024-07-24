//! In this module functions from [`commands module`][crate::commands]
//! executes using environment variables.
//!
//! If you don't like method with enivronment variables
//! for some reason or this module is not suitable for you,
//! you can easily write a executor yourself as you want,
//! sometimes looking into this module if something in [`notebook_api`][crate]
//! is not clear to you.
//!
//! ### How use commands
//! To use these commands you must enter
//! ```
//! cargo run -- `your-command`
//! ```
//! List of all commands:
//! * `add-note <notename>`- prompts to enter new note that will be added to the notebook under `notename`.
//! * `del-note <notename>` - deletes note with `notename` if it exist.
//! * `del-all` - deletes all total notes from the notebook.
//! * `upd-note <notename>` - prompts to enter a note that will be added instead old note in `notename`.
//! * `upd-notename <new notename>` - updates old notename to new `notename` of requested note.
//! * `display-note <notename>` - displays `notename`, `note` and note-`id` of requested note.
//! * If you did not specify which command to execute, then all total notes will be displayed.
use crate::commands::{add, del, del_all, display, display_all, upd, upd_notename};
use crate::errors::NotebookError;
use sqlx::{self, PgPool};
use std::{io, process};
use structopt::StructOpt;
use tracing::{event, Level};

/// Contains the command as `enum` from the environment variable to run it later.
///
/// This `struct` was created to conveniently store and execute commands on a notebook from enivronment variables.
/// More about these commands [here][crate::commands::execute_commands]
#[derive(StructOpt)]
pub struct NoteCommand {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}
#[derive(StructOpt)]
enum Command {
    AddNote {
        notename: String,
    },

    DelNote {
        notename: String,
    },
    DelAll,

    UpdNotename {
        notename: String,
        new_notename: String,
    },
    UpdNote {
        notename: String,
    },

    DisplayNote {
        notename: String,
    },
}

impl NoteCommand {
    /// Takes a command from enivronment variable as `enum` and saves it in `struct`.
    ///
    /// * will `Some(command)` if you selected any existing command
    /// * will `None` if you **didn't selected**/**selected a non-existent command**
    ///
    /// More about these commands [here][crate::commands::execute_commands]
    pub async fn new() -> Result<NoteCommand, structopt::clap::Error> {
        Ok(NoteCommand::from_args_safe()?)
    }
    /// Execute specifed command
    ///
    /// List of all commands:
    /// * `add-note`- prompts to enter a note that will be added to the notebook if no [errors][crate::errors] occurs.
    /// * `del-note` - deletes requested note if it exist using.
    /// * `del-all` - deletes all total notes from the notebook.
    /// * `upd-note` - update only `note` of requested note.
    /// * `upd-notename` - update only `notename` of requested note.
    /// * `display-note` - display `notename`, `note` and note-`id` of requested note.
    /// * If you did not specify which command to execute, then all total notes will be displayed.
    ///
    /// More about these commands [here][crate::commands::execute_commands]
    pub async fn execute_command(&self, pool: &PgPool) -> Result<(), NotebookError> {
        match self.cmd.as_ref() {
            Some(Command::AddNote { notename }) => {
                println!("Enter note you want to add into `{}`", notename);
                println!("(At the end of the note, enter `#endnote` on new line to finish writing the note)");

                let mut note = String::new();
                loop {
                    let mut check_end = String::new();

                    io::stdin().read_line(&mut check_end).unwrap_or_else(|e| {
                        event!(Level::DEBUG, "Problem to read line: {e}");

                        process::exit(1);
                    });

                    if check_end.contains("#endnote#") {
                        break;
                    }

                    note = note + check_end.as_str();
                }
                print!("Note to add into `{notename}`:\n{note}");

                add(&notename, &note, pool).await?;
            }

            Some(Command::DelNote { notename }) => {
                del(&notename, pool).await?;
            }

            Some(Command::DelAll) => {
                del_all(pool).await?;
            }

            Some(Command::UpdNotename {
                notename,
                new_notename,
            }) => {
                upd_notename(&notename, &new_notename, pool).await?;
            }

            Some(Command::UpdNote { notename }) => {
                println!(
                    "Enter note you want to add instead old note into `{}`",
                    notename
                );
                println!("(At the end of the note, enter `#endnote` on new line to finish writing the note)");

                let mut new_note = String::new();
                loop {
                    let mut check_end = String::new();

                    io::stdin().read_line(&mut check_end).unwrap_or_else(|e| {
                        event!(Level::DEBUG, "Problem to read `{}` line: {}", check_end, e);

                        process::exit(1);
                    });

                    if check_end.contains("#endnote#") {
                        break;
                    }

                    new_note = new_note + check_end.as_str();
                }
                print!("Note to add into `{notename}` instead old note:\n{new_note}");

                upd(&notename, &new_note, pool).await?;
            }

            Some(Command::DisplayNote { notename }) => {
                display(notename, pool).await?;
            }

            None => {
                display_all(pool).await?;
            }
        }
        Ok(())
    }
}
