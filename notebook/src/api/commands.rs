use std::env;

use crate::api::errors;

use errors::DataBaseError;

use anyhow::{self, Context};
use sqlx::{self, postgres::PgRow, PgPool};
use structopt::StructOpt;
use tracing::{event, Level};
use tracing_subscriber::fmt::format;

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
    UpdateNoteName {
        notename: String,
        new_notename: String,
    },
    UpdateNote {
        notename: String,
    },
}

// pub async fn print_data_from_db(pool: &PgPool) -> Result<PgRow, DataBaseError> {
//     let path = &env::var("DATABASE_URL").unwrap();
//     let db = PgPool::connect(&path).await?;

//     let row = sqlx::query(
//         "
// SELECT *
// FROM notebook
// WHERE
// ",
//     );

//     let all_data = sqlx::query!(
//         "
// SELECT *
// FROM notebook
// "
//     )
//     .fetch_all(&db)
//     .await?;

//     println!("Reading rows from db:\n{:?}", all_data);

//     Ok(row)
// }

impl NoteCommand {
    pub async fn new() -> anyhow::Result<Self> {
        anyhow::Ok(NoteCommand::from_args_safe().context("could not build struct from args")?)
    }
    pub async fn execute_command(&self, pool: &PgPool) -> Result<PgRow, DataBaseError> {
        match self.cmd.as_ref() {
            Some(Command::AddNote { notename }) => add_note(&notename, pool).await,

            Some(Command::DelNote { notename }) => delete_note(&notename, pool).await,

            Some(Command::UpdateNoteName {
                notename,
                new_notename,
            }) => update_notename(&notename, &new_notename, pool).await,

            Some(Command::UpdateNote { notename }) => update_note(&notename, pool).await,
            None => Ok(()),
        }
    }
}
async fn add_note<'a>(notename: &str, pool: &'a PgPool) -> Result<PgRow, DataBaseError> {
    let query = format!(
        "
INSERT INTO notebook
VALUES ( {}, {} )
            ",
        notename, ""
    );

    let row = match sqlx::query(&query).fetch_one(pool).await {
        Ok(row) => row,
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
            return Err(err.into());
        }
    };

    event!(Level::DEBUG, "Insert {:?} into notebook", notename);

    Ok(row)
}

async fn delete_note(notename: &str, pool: &PgPool) -> Result<PgRow, DataBaseError> {
    let query = format!(
        "
DELETE FROM notebook
WHERE note_name = {}
        ",
        notename
    );

    match sqlx::query(&query).fetch_one(pool).await {
        Ok(del_row) => {
            event!(Level::DEBUG, "Delete {:?} from notebook", notename);

            Ok(del_row)
        }
        Err(err) => Err(DataBaseError::Sqlx(err)),
    }
}

async fn update_note(notename: &str, pool: &PgPool) -> anyhow::Result<()> {
    Ok(())
}
async fn update_notename(
    notename: &str,
    notename_to_update: &str,
    pool: &PgPool,
) -> anyhow::Result<()> {
    Ok(())
}
