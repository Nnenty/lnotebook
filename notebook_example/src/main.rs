//! # Notebook_example
//! `Notebook_example` is simple example of use [`LNotebook API`][crate].

use anyhow;
use sqlx::{self, PgPool};
use tokio;

use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

use lnotebook_api::{get_db_url, NoteCommand};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("debug"))
        .init();

    // Get database URL from enivroment variable
    let db_url = get_db_url().await?;

    // Connecting to database
    let db = PgPool::connect(&db_url).await?;

    event!(Level::DEBUG, "Connect to db");

    // Converting terminal command to `enum` and save it in `NoteCommand`
    let a = NoteCommand::new().await?;
    // Execute the selected command
    a.execute_command(&db).await?;

    event!(Level::DEBUG, "Command executed");

    Ok(())
}
