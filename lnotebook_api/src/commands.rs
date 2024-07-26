//! This module contains functions that can be combined as you want and used to control a notebook.
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
/// async fn struct_example(pool: &PgPool) -> Result<(), NotebookError> {
///     // `add()` returns struct `Note` that we can use later as we wish
///     let row = add("early_sleep", "I'll go to bed early today", pool).await?;
///
///     assert_eq!("early_sleep", row.note_name);
///
///     Ok(())
/// }
pub struct Note {
    pub id: i32,
    pub note: Option<String>,
    pub note_name: String,
}

impl Note {
    /// Return field `note` as `&str`.
    ///
    /// If note is `Some()`, returns content of note as `&str`; else returns empty `&str`("")
    pub async fn note_str(&mut self) -> String {
        if let Some(some_note) = &self.note {
            some_note.to_owned()
        } else {
            "".to_owned()
        }
    }
}

/// Displays the requested note.
/// ### Returns
/// * Errors
///     * [`NotebookError::Sqlx`] error from [`sqlx::Error`]
pub async fn display(notename: &str, pool: &PgPool) -> Result<(), NotebookError> {
    let mut row = select_one(notename, pool).await?;
    let row_note = row.note_str().await;

    event!(
        Level::INFO,
        "Requested note:\nID: {}\nName: {}\nData:\n{}",
        row.id,
        row.note_name,
        row_note
    );

    Ok(())
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
            "\nID: {}:\nName: {}\nData:\n{}",
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
/// async fn add_example(pool: &PgPool) -> Result<(), NotebookError> {
///     // Retruns added note as struct `Note`
///     let row = add("add", "Added a some note so you don't forget", pool).await?;
///
///     assert_eq!("add", row.note_name);
///
///     Ok(())
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
                Level::INFO,
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
/// async fn delete_example(pool: &PgPool) -> Result<(), NotebookError> {
///     let row = add("bad_cat", "Buy new slippers. The old ones were ruined by the cat", pool).await?;
///
///     del(&row.note_name, pool).await?;
///
///     // Should return error because note `bad_cat` is not exist
///     display(&row.note_name, pool).await?;
///
///     Ok(())
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
                Level::INFO,
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
                    Level::INFO,
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

/// Clears the content of requested note.
/// ### Returns
/// * Errors
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
/// ### Example
/// ```rust,no run
/// async fn delete_example(pool: &PgPool) -> Result<(), NotebookError> {
///     add("clear_note", "meow meow meow meow", pool).await?;
///
///     clear("clear_note", pool).await?;
///     let row = select("clear_note", pool).await?;
///
///     assert_eq!("", row.note_str().await);
///
///     Ok(())
/// }
/// ```
pub async fn clear(notename: &str, pool: &PgPool) -> Result<(), NotebookError> {
    match sqlx::query!(
        "
UPDATE notebook
SET note = ''
WHERE note_name = $1
RETURNING note_name
        ",
        notename
    )
    .fetch_one(pool)
    .await
    {
        Ok(_) => {
            event!(Level::INFO, "Content of `{}` was cleared", notename);

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
/// async fn upd_example(pool: &PgPool) -> Result<(), NotebookError> {
///    add("wrong_note", "Thos is erong nlte", pool).await?;
///
///    // Returns updated note
///    let mut upd_row = upd("wrong_note", "This is NOT wrong note", pool).await?;
///
///    assert_eq!("This is NOT wrong note", upd_row.note_str().await);
///
///    Ok(())
/// }
/// ```
pub async fn upd(notename: &str, new_note: &str, pool: &PgPool) -> Result<Note, NotebookError> {
    match sqlx::query!(
        "
UPDATE notebook
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
            event!(Level::INFO, "Update `{}` data to:\n{}", notename, new_note,);

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
/// async fn upd_notename_example(pool: &PgPool) -> Result<(), NotebookError> {
///    add("wrlng_nptenAme", "", pool).await?;
///
///    // Returns updated notename
///    let upd_row = upd_notename("wrlng_nptenAme", "not_wrong_name", pool).await?;
///
///    assert_eq!("not_wrong_name", upd_row.note_name);
///
///    Ok(())
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
                Level::INFO,
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

/// Returns the requested note.
/// ### Returns
/// * Ok
///     * [Note]
/// * Errors
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
pub async fn select_one(notename: &str, pool: &PgPool) -> Result<Note, NotebookError> {
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

    Ok(Note {
        id: row.id,
        note: row.note,
        note_name: row.note_name,
    })
}
