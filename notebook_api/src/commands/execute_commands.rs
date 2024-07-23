use crate::commands::{add, del, del_all, display, display_all, upd, upd_notename};
use crate::errors::NotebookError;
use sqlx::{self, PgPool};
use std::{io, process};
use structopt::{clap, StructOpt};
use tracing::{event, Level};

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
    pub async fn new() -> Result<NoteCommand, clap::Error> {
        Ok(NoteCommand::from_args_safe()?)
    }
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

                    if check_end.contains("#endnote") {
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

                    if check_end.contains("#endnote") {
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
                use crate::commands::Note;

                async fn upd_notename_example(pool: &PgPool) -> Result<Note, NotebookError> {
                    add("wrlng_nptenAme", "", pool).await?;

                    // Returns updated notename
                    let upd_row = upd_notename("wrlng_nptenAme", "not_wrong_name", pool).await?;

                    assert_eq!("not_wrong_name", upd_row.note_name);

                    Ok(upd_row)
                }
                upd_notename_example(pool).await?;
            }
        }
        Ok(())
    }
}
