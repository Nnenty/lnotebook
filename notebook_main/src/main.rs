//! # Notebook_main
//! `Notebook_main` is binary file to run notebook_api, that creates
//! as example of use `notebook_api`. You can rewrite it as you need

use anyhow;
use sqlx::{self, PgPool};
use tokio;

use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

use notebook_api;
use notebook_api::{get_db_url, NoteCommand};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("debug"))
        .init();

    // Get database url from enivroment variable
    let db_url = get_db_url().await?;

    // Connecting to database
    let db = PgPool::connect(&db_url).await?;

    event!(Level::DEBUG, "Connect to db");

    // Converting command from environment variable to NoteCommand option
    let a = NoteCommand::new().await?;
    // Execute the selected command
    a.execute_command(&db).await?;

    event!(Level::DEBUG, "Command executed");

    Ok(())
}
