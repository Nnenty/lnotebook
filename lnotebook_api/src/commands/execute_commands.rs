//! In this module functions from [`commands module`][crate::commands]
//! executes using commands in terminal.
//!
//! If you don't like method with commands in terminal
//! for some reason or this module is not suitable for you,
//! you can easily write a executor yourself as you want,
//! sometimes looking into this module if something in [`notebook_api`][crate]
//! is not clear to you.
//!
//! ### How use commands
//! To begin you should write some code that will
//! create new [struct `NoteCommand`][NoteCommand] using [`NoteCommand::new`] and call [`NoteCommand::execute_command`].
//! For example, this is what the code from 'notebook_example' that meets the requirements looks like:
//! ```rust,no run
//! // --snip--
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!    tracing_subscriber::registry()
//!         .with(fmt::layer())
//!         .with(EnvFilter::new("debug"))
//!         .init();
//!
//!     // Get database URL from enivroment variable
//!     let db_url = get_db_url().await?;
//!
//!     // Connecting to database
//!     let db = PgPool::connect(&db_url).await?;
//!
//!     event!(Level::DEBUG, "Connect to db");
//!
//!     // Converting terminal command variable to NoteCommand option
//!     let a = NoteCommand::new().await?;
//!     // Execute the selected command
//!     a.execute_command(&db).await?;
//!
//!     event!(Level::DEBUG, "Command executed");
//!
//!     Ok(())
//! }
//! ```
//! To use these commands you must use:
//! ```bash
//! cargo run -- `your-command`
//! ```
//! List of all commands:
//! * `add-note <notename>` - will prompt to enter new note that will be added to the notebook under `notename`.
//! * `del-note <notename>` - deletes note with `notename` if it exist.
//! * `del-all` - deletes all total notes from the notebook.
//! * `upd-note <notename>` - will prompt to enter a note that will be added instead old note in `notename`.
//! * `upd-notename <new notename>` - updates old notename to new `notename` of requested note.
//! * `display-note <notename>` - displays `notename`, `note` and note-`id` of requested note.
//! * If you did not specify which command to execute, then all total notes will be displayed.
//!
//! #### Examples
//! Code under deletes 'unnecessary_note' if it exists:
//! ```bash
//! cargo run -- del-note unnecessary_note
//! ```
//!
//! Сommands such as `add-note` and `del-note`
//! will prompt you to enter a new note. To finish write note you
//! should write `#endnote#` at the end, as written in the tooltip.
//! For example the code below will update the 'passwords' note to
//! 'login: krutoy_4el\npassword: 123' if note exists:
//! ```bash
//! cargo run -- upd-note passwords
//!
//! # output
//! Enter note you want to add instead old note in `passwords`
//! (At the end of the note, enter `#endnote#` to finish writing the note):
//!
//! # input
//! login: krutoy_4el
//! password: 1234
//! #endnote#
//!
//! # output
//! Note to add into `passwords`:
//! login: krutoy_4el
//! password: 123
//! ```
//! Let's display full info about this note.
//!
//! If you did not specify which command to execute, then all total notes will be displayed.
//! You also can use `display-note` to display it, but for a variety we will do it like in the code below:
//! ```bash
//! cargo run
//!
//! # output
//! All notes in notebook:
//! ID: 1
//! Name: passwords
//! Data:
//! login: krutoy_4el
//! password: 123
//! ```
//! If there were more notes here, they would all be displayed, but since we only have one note, we only got that one.

use crate::commands::{add, del, del_all, display, display_all, upd, upd_notename};
use crate::errors::NotebookError;
use sqlx::{self, PgPool};
use std::{io, process};
use structopt::StructOpt;
use tracing::{event, Level};

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

/// Contains the command as `enum` from terminal to run it later.
///
/// This `struct` was created to conveniently store and execute commands on a notebook from terminal commands.
/// More about commands for which this structure was created [here][crate::commands::execute_commands].
#[derive(StructOpt)]
pub struct NoteCommand {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}
impl NoteCommand {
    /// Convert a command from terminal to `enum` and saves it in [struct `NoteCommand`][NoteCommand].
    ///
    /// Command stores in [`NoteCommand`] as `Option<Command>` and will be:
    /// * `Some(Command)` if you selected any existing command
    /// * `None` if you **didn't selected**/**selected a non-existent command**
    ///
    /// More about commands [here][crate::commands::execute_commands].
    pub async fn new() -> Result<NoteCommand, structopt::clap::Error> {
        Ok(NoteCommand::from_args_safe()?)
    }
    /// Execute specifed command.
    ///
    /// List of all commands:
    /// * `add-note <notename>`- prompts to enter new note that will be added to the notebook under `notename`.
    /// * `del-note <notename>` - deletes note with `notename` if it exist.
    /// * `del-all` - deletes all total notes from the notebook.
    /// * `upd-note <notename>` - prompts to enter a note that will be added instead old note in `notename`.
    /// * `upd-notename <new notename>` - updates old notename to `new notename` of requested note.
    /// * `display-note <notename>` - displays `notename`, `note` and note-`id` of requested note.
    /// * If you did not specify which command to execute, then all total notes will be displayed.
    ///
    /// More about these commands [here][crate::commands::execute_commands].
    pub async fn execute_command(&self, pool: &PgPool) -> Result<(), NotebookError> {
        match self.cmd.as_ref() {
            Some(Command::AddNote { notename }) => {
                println!("Current note in `{}`", notename);
                println!("Enter note you want to add into `{}`", notename);
                println!("(At the end of the note, enter `#endnote#` to finish writing the note):");

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
                    "Enter note you want to add instead old note in `{}`",
                    notename
                );
                println!("(At the end of the note, enter `#endnote#` to finish writing the note):");

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
