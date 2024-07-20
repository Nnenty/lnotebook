use std::env;

use anyhow::{self, Ok};

use sqlx::{self, PgPool};
use tokio;

use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

mod api;
use api::NoteCommand;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new("debug"))
        .init();

    let db = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    event!(Level::DEBUG, "Connect to db");

    let a = NoteCommand::new().await?;
    a.execute_command(&db).await?;

    event!(Level::DEBUG, "Command executed");

    Ok(())
}
