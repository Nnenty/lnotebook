use crate::commands::{
    add_note, delete_all_notes, delete_note, print_all_data, print_note, update_note,
    update_notename,
};
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
    Addnote {
        notename: String,
    },
    Delnote {
        notename: String,
    },
    Delall,
    Updnotename {
        notename: String,
        new_notename: String,
    },
    Updnote {
        notename: String,
    },
    Printnote {
        notename: String,
    },
}

impl NoteCommand {
    pub async fn new() -> Result<NoteCommand, clap::Error> {
        Ok(NoteCommand::from_args_safe()?)
    }
    pub async fn execute_command(&self, pool: &PgPool) -> Result<(), NotebookError> {
        match self.cmd.as_ref() {
            Some(Command::Addnote { notename }) => {
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
                print!("Note to add into {notename}:\n{note}");

                add_note(&notename, &note, pool).await?;
            }

            Some(Command::Delnote { notename }) => {
                delete_note(&notename, pool).await?;
            }

            Some(Command::Delall) => {
                delete_all_notes(pool).await?;
            }

            Some(Command::Updnotename {
                notename,
                new_notename,
            }) => {
                update_notename(&notename, &new_notename, pool).await?;
            }

            Some(Command::Updnote { notename }) => {
                println!(
                    "Are you sure want to update data of `{}`?\n(enter `y` or `n`)",
                    notename
                );

                let mut yes_no = String::new();
                io::stdin().read_line(&mut yes_no).unwrap_or_else(|e| {
                    event!(Level::DEBUG, "Problem to read `{}` line: {}", yes_no, e);

                    process::exit(1);
                });

                if yes_no.trim().to_lowercase().contains("y") {
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

                    update_note(&notename, &new_note, pool).await?;
                } else {
                    println!("You refused to update note");
                }
            }

            Some(Command::Printnote { notename }) => {
                print_note(notename, pool).await?;
            }

            None => {
                use crate::commands::Notebook;

                async fn print_example(pool: &PgPool) -> Result<Notebook, NotebookError> {
                    let row = add_note("early_sleep", "I'll go to bed early today", pool).await?;

                    // Print and return `Notebook`
                    let row = print_note(&row.note_name, pool).await?;

                    assert_eq!("early_sleep", row.note_name);

                    Ok(row)
                }

                print_example(pool).await?;
            }
        }
        Ok(())
    }
}
