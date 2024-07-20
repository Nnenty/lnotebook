use sqlx::PgPool;

use super::*;

#[tokio::test]
pub async fn test_del_note() -> anyhow::Result<()> {
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
        notename: note_for_delete,
    }
    .into())
}
