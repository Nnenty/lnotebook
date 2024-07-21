use anyhow;

use sqlx::{self, PgPool};
use tokio;

use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

mod api;
use api::{get_db_url, NoteCommand};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("debug"))
        .init();

    let db_url = get_db_url().await?;

    let db = PgPool::connect(&db_url).await?;

    event!(Level::DEBUG, "Connect to db");

    let a = NoteCommand::new().await?;
    a.execute_command(&db).await?;

    event!(Level::DEBUG, "Command executed");

    Ok(())
}
