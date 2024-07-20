use crate::api::errors;

use errors::DataBaseError;

use sqlx::{self, postgres::PgRow, PgPool, Row};
use structopt::{clap, StructOpt};
use tracing::{event, Level};

#[derive(StructOpt)]
pub struct NoteCommand {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}
#[derive(StructOpt)]
pub enum Command {
    AddNote {
        notename: String,
    },
    DelNote {
        notename: String,
    },
    UpdNoteName {
        notename: String,
        new_notename: String,
    },
    UpdNote {
        notename: String,
        new_note: String,
    },
}

async fn return_first_row(pool: &PgPool) -> Result<PgRow, DataBaseError> {
    let row = sqlx::query(
        "
SELECT * 
FROM notebook
        ",
    )
    .fetch_one(pool)
    .await?;

    let display_row = row.try_column(0)?;

    println!("All notes in notebook:\n{:?}", display_row);

    Ok(row)
}

impl NoteCommand {
    pub async fn new() -> Result<NoteCommand, clap::Error> {
        Ok(NoteCommand::from_args_safe()?)
    }
    pub async fn execute_command(&self, pool: &PgPool) -> Result<PgRow, DataBaseError> {
        match self.cmd.as_ref() {
            Some(Command::AddNote { notename }) => add_note(&notename, pool).await,

            Some(Command::DelNote { notename }) => delete_note(&notename, pool).await,

            Some(Command::UpdNoteName {
                notename,
                new_notename,
            }) => update_notename(&notename, &new_notename, pool).await,
            Some(Command::UpdNote { notename, new_note }) => {
                update_note(&notename, &new_note, pool).await
            }

            None => return_first_row(pool).await,
        }
    }
}
async fn add_note(notename: &str, pool: &PgPool) -> Result<PgRow, DataBaseError> {
    let query = format!(
        "
INSERT INTO notebook (note_name, note)
VALUES ( '{notename}', '{}' )
RETURNING (note_name, note)
        ",
        ""
    );

    match sqlx::query(&query).fetch_one(pool).await {
        Ok(row) => {
            event!(
                Level::DEBUG,
                "Insert note with name `{}` into notebook",
                notename
            );
            Ok(row)
        }
        Err(err) => {
            if let Some(db_err) = err.as_database_error() {
                if let Some(code) = db_err.code() {
                    if code == "23505" {
                        return Err(DataBaseError::AlreadyTaken {
                            notename: notename.to_owned(),
                        });
                    }
                }
            }
            Err(err.into())
        }
    }
}

async fn delete_note(notename: &str, pool: &PgPool) -> Result<PgRow, DataBaseError> {
    let query = format!(
        "
DELETE FROM notebook
WHERE note_name = '{}'
RETURNING note_name
        ",
        notename
    );

    match sqlx::query(&query).fetch_one(pool).await {
        Ok(del_row) => {
            event!(
                Level::DEBUG,
                "Delete note with name `{}` from notebook",
                notename
            );

            Ok(del_row)
        }
        Err(err) => Err(DataBaseError::Sqlx(err)),
    }
}

async fn update_note(
    notename: &str,
    new_note: &str,
    pool: &PgPool,
) -> Result<PgRow, DataBaseError> {
    let query = format!(
        "
UPDATE notebook
SET note = '{}'
WHERE note_name = '{}'
RETURNING (note_name, note)
        ",
        new_note, notename
    );

    match sqlx::query(&query).fetch_one(pool).await {
        Ok(upd_row) => {
            let n = upd_row.try_column(0)?;

            event!(
                Level::DEBUG,
                "Update `{}` note from {:?} to {}",
                notename,
                n,
                new_note
            );

            Ok(upd_row)
        }
        Err(err) => Err(DataBaseError::Sqlx(err)),
    }
}
async fn update_notename(
    notename: &str,
    new_notename: &str,
    pool: &PgPool,
) -> Result<PgRow, DataBaseError> {
    let query = format!(
        "
UPDATE notebook
SET note_name = '{}'
WHERE note_name = '{}'
RETURNING note_name
        ",
        new_notename, notename
    );

    match sqlx::query(&query).fetch_one(pool).await {
        Ok(upd_row) => {
            let new_notename = upd_row.try_column(0)?;

            event!(
                Level::DEBUG,
                "Update notename from {} to {:?}",
                notename,
                new_notename
            );

            Ok(upd_row)
        }
        Err(err) => Err(DataBaseError::Sqlx(err)),
    }
}
