use sqlx::PgPool;
use std::env;

use crate::api::commands::{add_note, delete_note};

#[tokio::test]
async fn test_delnote() -> anyhow::Result<()> {
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

    Err()
}

#[tokio::test]
async fn test_updnote() {
    let db = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    let note_to_update;
}
