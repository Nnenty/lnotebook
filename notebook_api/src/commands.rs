//! This module containing commands you can run to control the notebook.

pub mod execute_commands;
use crate::errors;
use errors::NotebookError;

use sqlx::{self, PgPool};
use tracing::{event, Level};

/// This is a `struct` that containing information about notes.
///
/// This `struct` is returned by `functions` from [`command` module][`crate::commands`]:
/// * [`add`]
/// * [`upd`]
/// * [`upd_notename`]
/// * [`display`]
/// ### Example
/// ```rust,no run
/// async fn struct_example(pool: &PgPool) -> Result<Note, NotebookError> {
///     // `add()` returns struct `Note` that we can use later as we wish
///     let row = add("early_sleep", "I'll go to bed early today", pool).await?;
///
///     assert_eq!("early_sleep", row.note_name);
///
///     Ok(row)
/// }
pub struct Note {
    pub id: i32,
    pub note: Option<String>,
    pub note_name: String,
}

/// Displays and return the requested note.
/// ### Returns
/// * Ok
///     * [`Note`]
/// * Errors
///     * [`NotebookError::Sqlx`] error from [`sqlx::Error`]
/// ### Example
/// ```rust,no run
/// async fn display_example(pool: &PgPool) -> Result<Note, NotebookError> {
///    add("med_evn", "Dont forget to meditate in the evening", pool).await?;
///
///    // Display and return `Note` that was displayed
///    let displayed_row = display("med_evn", pool).await?;
///
///    assert_eq!("med_evn", displayed_row.note_name);
///
///    Ok(displayed_row)
/// }
/// ```
pub async fn display(notename: &str, pool: &PgPool) -> Result<Note, NotebookError> {
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

    let row_note = if let Some(n) = &row.note { n } else { "" };

    event!(
        Level::INFO,
        "Requested note:\nID: {}\nName: {}\nData:\n{}",
        row.id,
        row.note_name,
        row_note
    );

    Ok(Note {
        id: row.id,
        note: row.note,
        note_name: row.note_name,
    })
}

/// Displays all total notes in notebook.
/// ### Returns
/// * Errors
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
pub async fn display_all(pool: &PgPool) -> Result<(), NotebookError> {
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
        let row_note = if let Some(n) = &row.note { n } else { "" };

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

/// Adds and returns a new note to notebook.
/// ### Returns
/// * Ok
///     * [Note] that was added into notebook
/// * Errors
///     * [`NotebookError::AlreadyTaken`] error if a note with the same name already exists
///     * [`NotebookError::Sqlx`] error from [`sqlx::Error`]
/// if any other [`sqlx::Error`] occurs
/// ### Example
/// ```rust,no run
/// async fn add_example(pool: &PgPool) -> Result<Note, NotebookError> {
///     // Retruns added note as struct `Note`
///     let row = add("add", "Added a some note so you don't forget", pool).await?;
///
///     assert_eq!("add", row.note_name);
///
///     Ok(row)
/// }
/// ```
pub async fn add(notename: &str, note: &str, pool: &PgPool) -> Result<Note, NotebookError> {
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
            Ok(Note {
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

/// Deletes the requested note.
/// ### Returns
/// * Errors
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
/// ### Example
/// ```rust,no run
/// async fn delete_example(pool: &PgPool) -> Result<Note, NotebookError> {
///     let row = add("bad_cat", "Buy new slippers. The old ones were ruined by the cat", pool).await?;
///
///     del(&row.note_name, pool).await?;
///
///     // Should return error because note `bad_cat` is not exist
///     display(&row.note_name, pool).await?;
///
///     Ok(row)
/// }
/// ```
pub async fn del(notename: &str, pool: &PgPool) -> Result<(), NotebookError> {
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
            let row_note = if let Some(n) = &row.note { n } else { "" };

            event!(
                Level::DEBUG,
                "Deleteing note:\nID: {}\nName: {}\nData:\n{}",
                row.id,
                notename,
                row_note
            );

            Ok(())
        }
        Err(err) => Err(NotebookError::Sqlx(err)),
    }
}

/// Deletes all total notes in notebook.
/// ### Returns
/// * Errors
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
/// ### Example
/// ```rust,no run
/// async fn delete_all_example(pool: &PgPool) -> Result<(), NotebookError> {
///     // Adding new notes
///     add(
///         "bad_cat",
///         "Buy new slippers. the old ones were ruined by the cat",
///         pool,
///     )
///     .await?;
///     add(
///         "cool_cat",
///         "Don't forget to post a photo of my cool cat",
///         pool,
///     )
///     .await?;
///     add("empty", "", pool).await?;
///
///     del_all(pool).await?;
///
///     // Should display empty list
///     display_all(pool).await?;
///
///     Ok(())
/// }
/// ```
pub async fn del_all(pool: &PgPool) -> Result<(), NotebookError> {
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
                let row_note = if let Some(n) = &row.note { n } else { "" };

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

/// Updates note and returns updated note.
/// ### Returns
/// * Ok
///     * [Note] that was updated
/// * Errors
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
/// ### Example
/// ```rust,no run
/// async fn upd_example(pool: &PgPool) -> Result<Note, NotebookError> {
///    add("wrong_note", "Thos is erong nlte", pool).await?;
///
///    // Returns updated note
///    let upd_row = upd("wrong_note", "This is NOT wrong note", pool).await?;
///    // Parcing `Some()`
///    let upd_row_note = if let Some(n) = &upd_row.note { n } else { "" };
///
///    assert_eq!("This is NOT wrong note", upd_row_note);
///
///    Ok(upd_row)
/// }
/// ```
pub async fn upd(notename: &str, new_note: &str, pool: &PgPool) -> Result<Note, NotebookError> {
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

            Ok(Note {
                id: upd_row.id,
                note_name: upd_row.note_name,
                note: upd_row.note,
            })
        }
        Err(err) => Err(NotebookError::Sqlx(err)),
    }
}

/// Updates notename and returns note that name was updated.
/// ### Returns
/// * Ok
///     * [Note] that name was updated
/// * Errors
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
/// ### Example
/// ```rust,no run
/// async fn upd_notename_example(pool: &PgPool) -> Result<Note, NotebookError> {
///    add("wrlng_nptenAme", "", pool).await?;
///
///    // Returns updated notename
///    let upd_row = upd_notename("wrlng_nptenAme", "not_wrong_name", pool).await?;
///
///    assert_eq!("not_wrong_name", upd_row.note_name);
///
///    Ok(upd_row)
/// }
/// ```
pub async fn upd_notename(
    notename: &str,
    new_notename: &str,
    pool: &PgPool,
) -> Result<Note, NotebookError> {
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

            Ok(Note {
                id: upd_row.id,
                note_name: upd_row.note_name,
                note: upd_row.note,
            })
        }
        Err(err) => Err(NotebookError::Sqlx(err)),
    }
}
