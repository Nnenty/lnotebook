use std::env;

use anyhow::{self, Context, Ok};
use sqlx::{self, Error, PgPool};
use structopt::StructOpt;
use tokio;

use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum DataBaseError {
    #[error("The note-name `{note_name}` is already taken; try use another note-name")]
    AlreadyTaken { note_name: String },

    #[error("Error to delete `{note_name}`")]
    DeleteError { note_name: String },
}

#[derive(StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    cmd: Option<NoteCommand>,
}
#[derive(StructOpt)]
pub enum NoteCommand {
    AddNote { note_name: String },
    DelNote { note_name: String },
    UpdateNote { note_name: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("debug"))
        .init();

    let db = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let a = Args::new().await?;
    a.execute_command(&db).await?;

    Ok(())
}
async fn add_note(note_name: &str, pool: &PgPool) -> anyhow::Result<()> {
    if let Some(_) = sqlx::query!(
        "
SELECT *
FROM notebook
WHERE note_name = $1
        ",
        note_name
    )
    .fetch_optional(pool)
    .await?
    {
        return Err(DataBaseError::AlreadyTaken {
            note_name: note_name.to_owned(),
        }
        .into());
    }

    let row = sqlx::query!(
        "
INSERT INTO notebook
VALUES ( $1, $2 )
RETURNING note_name
        ",
        note_name,
        ""
    )
    .fetch_one(pool)
    .await?;

    event!(Level::DEBUG, "Insert {:?} into notebook", row.note_name);

    Ok(())
}

async fn delete_note(note_name: &str, pool: &PgPool) -> anyhow::Result<()> {
    if let Some(del_row) = sqlx::query!(
        "
DELETE FROM notebook
WHERE note_name = $1
RETURNING note_name
        ",
        note_name
    )
    .fetch_optional(pool)
    .await?
    {
        event!(Level::DEBUG, "Delete {:?} from notebook", del_row.note_name);

        return Ok(());
    }

    Err(Error::RowNotFound.into())
}
async fn update_note(note_name: &str, pool: &PgPool) -> anyhow::Result<()> {
    Ok(())
}

impl Args {
    pub async fn new() -> anyhow::Result<Self> {
        anyhow::Ok(Args::from_args_safe().context("could not build struct from args")?)
    }
    pub async fn execute_command(&self, pool: &PgPool) -> anyhow::Result<()> {
        match self.cmd.as_ref() {
            Some(NoteCommand::AddNote { note_name }) => add_note(note_name, pool).await,
            Some(NoteCommand::DelNote { note_name }) => delete_note(note_name, pool).await,
            Some(NoteCommand::UpdateNote { note_name }) => update_note(note_name, pool).await,
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Error;

    use super::*;

    #[tokio::test]
    async fn print_data_from_db() -> anyhow::Result<()> {
        let db = PgPool::connect(&env::var("DATABASE_URL")?).await?;

        let ret = sqlx::query!(
            "
SELECT *
FROM notebook
            "
        )
        .fetch_all(&db)
        .await?;

        println!("Reading rows from db:\n{:?}", ret);

        Ok(())
    }

    #[tokio::test]
    async fn test_del_note() -> anyhow::Result<()> {
        let db = PgPool::connect(&env::var("DATABASE_URL")?).await?;
        let note_for_delete = "test_note_test_note".to_owned();

        add_note(&note_for_delete, &db).await?;
        delete_note(&note_for_delete, &db)
            .await
            .unwrap_or_else(|e| {
                println!("Could not delete note: {e}");
            });

        if let None = sqlx::query!(
            "
SELECT *
FROM notebook
WHERE note_name = $1
            ",
            note_for_delete
        )
        .fetch_optional(&db)
        .await?
        {
            println!("{note_for_delete} was deleted from db");

            return Ok(());
        }

        Err(DataBaseError::DeleteError {
            note_name: note_for_delete,
        }
        .into())
    }
}
