use crate::api::DataBaseError;
use crate::api::{
    add_note, delete_all_notes, delete_note, print_all_data, update_note, update_notename,
};

use sqlx::{self, PgPool};
use structopt::{clap, StructOpt};

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
    UpdNoteName {
        notename: String,
        new_notename: String,
    },
    UpdNote {
        notename: String,
        new_note: String,
    },
}

impl NoteCommand {
    pub async fn new() -> Result<NoteCommand, clap::Error> {
        Ok(NoteCommand::from_args_safe()?)
    }
    pub async fn execute_command(&self, pool: &PgPool) -> Result<(), DataBaseError> {
        match self.cmd.as_ref() {
            Some(Command::AddNote { notename }) => {
                add_note(&notename, pool).await?;
            }

            Some(Command::DelNote { notename }) => {
                delete_note(&notename, pool).await?;
            }

            Some(Command::DelAll) => {
                delete_all_notes(pool).await?;
            }

            Some(Command::UpdNoteName {
                notename,
                new_notename,
            }) => {
                update_notename(&notename, &new_notename, pool).await?;
            }

            Some(Command::UpdNote { notename, new_note }) => {
                update_note(&notename, &new_note, pool).await?;
            }

            None => {
                print_all_data(pool).await?;
            }
        };

        Ok(())
    }
}
