//! Module containing commands you can run to control the notebook

pub mod execute_commands;
pub use execute_commands::NoteCommand;

use crate::errors;
use errors::NotebookError;

use sqlx::{self, PgPool};
use tracing::{event, Level};

pub struct Notebook {
    pub id: i32,
    pub note: Option<String>,
    pub note_name: String,
}

pub async fn print_note(notename: &str, pool: &PgPool) -> Result<Notebook, NotebookError> {
    let row = sqlx::query!(
        "
SELECT *
FROM notebook
WHERE note_name = $1
        ",
        notename
    )
    .fetch_one(pool)
    .await?;

    let row_note = if let Some(n) = &row.note {
        n
    } else {
        "#NONE-DATA#"
    };

    event!(
        Level::INFO,
        "Requested note:\nID: {}\nName: {}\nData:\n{}",
        row.id,
        row.note_name,
        row_note
    );

    Ok(Notebook {
        id: row.id,
        note: row.note,
        note_name: row.note_name,
    })
}

pub async fn print_all_data(pool: &PgPool) -> Result<(), NotebookError> {
    let rows = sqlx::query!(
        "
SELECT * 
FROM notebook
        "
    )
    .fetch_all(pool)
    .await?;

    event!(Level::INFO, "All notes in notebook:");
    rows.iter().for_each(|row| {
        let row_note = if let Some(n) = &row.note {
            n
        } else {
            "#NONE-DATA#"
        };

        event!(
            Level::INFO,
            "\nID: {}:\nName: {}\nData: {}",
            row.id,
            row.note_name,
            row_note
        );
    });

    Ok(())
}

pub async fn add_note(
    notename: &str,
    note: &str,
    pool: &PgPool,
) -> Result<Notebook, NotebookError> {
    match sqlx::query!(
        "
INSERT INTO notebook (note_name, note)
VALUES ( $1, $2 )
RETURNING id, note_name, note
        ",
        notename,
        note
    )
    .fetch_one(pool)
    .await
    {
        Ok(row) => {
            event!(
                Level::DEBUG,
                "Insert note with name `{}` with data `{}` into notebook",
                notename,
                note
            );
            Ok(Notebook {
                id: row.id,
                note: row.note,
                note_name: row.note_name,
            })
        }
        Err(err) => {
            if let Some(db_err) = err.as_database_error() {
                if let Some(code) = db_err.code() {
                    if code == "23505" {
                        return Err(NotebookError::AlreadyTaken {
                            notename: notename.to_owned(),
                        });
                    }
                }
            }
            Err(err.into())
        }
    }
}

pub async fn delete_note(notename: &str, pool: &PgPool) -> Result<(), NotebookError> {
    match sqlx::query!(
        "
DELETE FROM notebook
WHERE note_name = $1
RETURNING id, note_name, note
        ",
        notename
    )
    .fetch_one(pool)
    .await
    {
        Ok(row) => {
            let row_note = if let Some(n) = &row.note {
                n
            } else {
                "#NONE-DATA#"
            };

            event!(
                Level::DEBUG,
                "Delete note:\nID: #{}\nName: {}\nData:\n{}",
                row.id,
                notename,
                row_note
            );

            Ok(())
        }
        Err(err) => Err(NotebookError::Sqlx(err)),
    }
}

pub async fn delete_all_notes(pool: &PgPool) -> Result<(), NotebookError> {
    match sqlx::query!(
        "
DELETE FROM notebook
RETURNING id, note_name, note
        "
    )
    .fetch_all(pool)
    .await
    {
        Ok(del_rows) => {
            del_rows.iter().for_each(|row| {
                let row_note = if let Some(n) = &row.note {
                    n
                } else {
                    "#NONE-DATA#"
                };

                event!(
                    Level::DEBUG,
                    "Deleting ID: {}; Name: {}; Data:\n{}",
                    row.id,
                    row.note_name,
                    row_note
                )
            });

            Ok(())
        }
        Err(err) => Err(NotebookError::Sqlx(err)),
    }
}

pub async fn update_note(
    notename: &str,
    new_note: &str,
    pool: &PgPool,
) -> Result<Notebook, NotebookError> {
    match sqlx::query!(
        "UPDATE notebook
        SET note = $1
        WHERE note_name = $2
        RETURNING id, note_name, note
        ",
        new_note,
        notename,
    )
    .fetch_one(pool)
    .await
    {
        Ok(upd_row) => {
            event!(Level::DEBUG, "Update `{}` data to:\n{}", notename, new_note,);

            Ok(Notebook {
                id: upd_row.id,
                note_name: upd_row.note_name,
                note: upd_row.note,
            })
        }
        Err(err) => Err(NotebookError::Sqlx(err)),
    }
}
pub async fn update_notename(
    notename: &str,
    new_notename: &str,
    pool: &PgPool,
) -> Result<Notebook, NotebookError> {
    match sqlx::query!(
        "
UPDATE notebook
SET note_name = $1
WHERE note_name = $2
RETURNING id, note_name, note
        ",
        new_notename,
        notename
    )
    .fetch_one(pool)
    .await
    {
        Ok(upd_row) => {
            event!(
                Level::DEBUG,
                "Update notename\nFrom: {}\nTo: {}",
                notename,
                new_notename
            );

            Ok(Notebook {
                id: upd_row.id,
                note_name: upd_row.note_name,
                note: upd_row.note,
            })
        }
        Err(err) => Err(NotebookError::Sqlx(err)),
    }
}
