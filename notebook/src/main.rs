use std::env;

use anyhow::{self, Context, Ok};
use sqlx::{self, PgPool};
use structopt::StructOpt;
use tokio;
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

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
    let p = sqlx::query!(
        "
INSERT INTO notebook
VALUES ( $1, $2 )
        ",
        note_name,
        ""
    )
    .execute(pool)
    .await
    .unwrap();

    event!(Level::DEBUG, "Insert {note_name} into notebook");

    Ok(())
}
async fn delete_note(note_name: &str, pool: &PgPool) -> anyhow::Result<()> {
    Ok(())
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
    use super::*;

    #[tokio::test]
    // cargo test print_data_from_db -- --nocapture
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

        println!("Reading rows from db:\n{:?} from db", ret);

        Ok(())
    }
}
