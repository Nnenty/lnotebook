//! In this module functions from [`commands` module][crate::commands]
//! executes using commands in CLI.
//!
//! ### About module
//! First of all, the [`LNotebook`][crate] was created so that you can
//! quickly and easily write your own notebook. This module was created so that you can run the notebook
//! right away without manually using the functions from the [`commands` module][crate::commands],
//! and it also demonstrates how you could use [`LNotebook`][crate]. So if this way of using
//! the [`LNotebook`][crate] doesn't suit you, just write your own way to use it.
//!
//! ### How use commands
//! To begin you should write some code that will
//! call [`NoteCommand::new`] and [`NoteCommand::execute_command`].
//! For example, this is what the code from [`notebook_example`](https://github.com/Nnenty/lnotebook/tree/master/notebook_example)
//! that meets the requirements looks like:
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
//!     // Converting CLI command variable to NoteCommand option
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
//! ##### List of all commands you can call from CLI:
//! * `add-note <notename>` - will prompt to enter new note that will be added to the notebook under `notename`.
//! * `del-note <notename>` - deletes note with `notename` if it exist.
//! * `del-all` - deletes all total notes from the notebook.
//! * `clear-note <notename>` - clears content of `notename`
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
//! Сommands such as `add-note` and `upd-note`
//! will prompt you to enter a new note. To finish write note you
//! should write `#endnote#` at the end, as written in the tooltip.
//! For example the code below will update the 'passwords' content to
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
//! password: 1234#endnote#
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

use crate::commands::{
    add, clear, del, del_all, display, display_all, select_one, upd, upd_notename,
};
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
    ClearNote {
        notename: String,
    },

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

/// Contains the command as `enum` from CLI to run it later.
///
/// This `struct` was created to conveniently store and execute commands on a notebook from CLI commands.
/// More about commands for which this structure was created [here][crate::commands::execute_commands].
#[derive(StructOpt)]
pub struct NoteCommand {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}
impl NoteCommand {
    /// Convert a command from CLI to `enum` and saves it in [struct `NoteCommand`][NoteCommand].
    ///
    /// Command stores in [`NoteCommand`] as `Option<Command>` and will be:
    /// * `Some(Command)` if you selected any existing command
    /// * `None` if you **didn't selected**/**selected a non-existent command**
    ///
    /// Read about CLI commands [here][crate::commands::execute_commands].
    pub async fn new() -> Result<NoteCommand, structopt::clap::Error> {
        Ok(NoteCommand::from_args_safe()?)
    }
    /// Execute specifed command.
    ///
    /// [List of all CLI commands.](https://docs.rs/lnotebook/latest/lnotebook/commands/execute_commands/index.html#list-of-all-commands-you-can-call-from-CLI).
    ///
    /// Read about CLI commands [here][crate::commands::execute_commands].
    pub async fn execute_command(&self, pool: &PgPool) -> Result<(), NotebookError> {
        match self.cmd.as_ref() {
            Some(Command::AddNote { notename }) => {
                println!("Enter note you want to add into `{}`", notename);
                println!("(At the end of the note, enter `#endnote#` to finish writing the note):");

                let mut note = String::new();
                loop {
                    let mut note_part = String::new();

                    io::stdin().read_line(&mut note_part).unwrap_or_else(|e| {
                        event!(Level::DEBUG, "Problem to read line: {e}");

                        process::exit(1);
                    });

                    if note_part.contains("#endnote#") {
                        delete_end(&mut note_part, "#endnote#").await;
                        note = note + note_part.as_str();

                        break;
                    } else {
                        note = note + note_part.as_str();
                    }
                }
                println!("Note to add into `{notename}`:\n{note}");

                add(&notename, &note, pool).await?;
            }

            Some(Command::DelNote { notename }) => {
                del(&notename, pool).await?;
            }

            Some(Command::DelAll) => {
                del_all(pool).await?;
            }

            Some(Command::ClearNote { notename }) => {
                clear(notename, pool).await?;
            }

            Some(Command::UpdNotename {
                notename,
                new_notename,
            }) => {
                upd_notename(&notename, &new_notename, pool).await?;
            }

            Some(Command::UpdNote { notename }) => {
                println!(
                    "Current content of `{}`:\n{}",
                    notename,
                    select_one(notename, pool).await?.note_str().await
                );

                println!(
                    "Enter note you want to add instead old note in `{}`",
                    notename
                );
                println!("(At the end of the note, enter `#endnote#` to finish writing the note):");

                let mut note = String::new();
                loop {
                    let mut note_part = String::new();

                    io::stdin().read_line(&mut note_part).unwrap_or_else(|e| {
                        event!(Level::DEBUG, "Problem to read line: {e}");

                        process::exit(1);
                    });

                    if note_part.contains("#endnote#") {
                        delete_end(&mut note_part, "#endnote#").await;
                        note = note + note_part.as_str();

                        break;
                    } else {
                        note = note + note_part.as_str();
                    }
                }
                println!("Note to add into `{notename}` instead old note:\n{note}");

                upd(&notename, &note, pool).await?;
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
async fn delete_end(source: &mut String, end: &str) -> String {
    let _: Vec<_> = source
        .to_owned()
        .char_indices()
        .map(|(i, _)| {
            // length of end
            let len = i + end.len();

            if source.contains(end) {
                if &source[i..len] == end {
                    // delete end from source and extra information behind it
                    source.drain(i..);
                }
            }
        })
        .collect();

    source.to_owned()
}
