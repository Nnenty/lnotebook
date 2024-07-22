//! This module containing commands you can run to control the notebook

pub mod execute_commands;
use crate::errors;
use errors::NotebookError;
pub use execute_commands::NoteCommand;

use sqlx::{self, PgPool};
use tracing::{event, Level};

/// This is `struct` that returned by functions in [`command` module][`crate::commands`]
///
/// `functions` that return it:
/// * [`add_note`][add_note]
/// * [`update_note`][update_note]
/// * [`update_notename`][update_notename]
/// * [`print_note`][print_note]
/// ### Example
/// ```rust,no run
/// async fn struct_example(pool: &PgPool) -> Result<Notebook, NotebookError> {
///     // `add_note()` returns `struct Notebook` that we can use as we wish
///     let row = add_note("early_sleep", "I'll go to bed early today", pool).await?;
///
///     assert_eq!("early_sleep", row.note_name);
///
///     Ok(row)
/// }
pub struct Notebook {
    pub id: i32,
    pub note: Option<String>,
    pub note_name: String,
}

/// Displays the requested note
/// ### Returns
/// * Ok
///     * Returns the displayed [note of `Notebook` type][Notebook]
/// * Errors
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
/// ### Example
/// ```rust,no run
/// async fn print_example(pool: &PgPool) -> Result<Notebook, NotebookError> {
///     let row = add_note("med_evn", "Meditate in the evening", pool).await?;
///
///     // Display and return `Notebook` that was displayed
///     let check_similar = print_note(&row.note_name, pool).await?;
///
///     assert_eq!(check_similar.note_name, row.note_name);
///
///     Ok(row)
/// }
/// ```
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

    let row_note = if let Some(n) = &row.note { n } else { "" };

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

/// Displays all total notes in notebook
/// ### Returns
/// * Errors
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
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

/// Adds a new note to notebook
/// ### Returns
/// * Ok
///     * [note of `Notebook` type][Notebook] that was added into notebook
/// * Errors
///     * [`NotebookError::AlreadyTaken`] error if a note with the same name already exists
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
/// if any other [`sqlx::Error`] occurs
/// ### Example
/// ```rust,no run
/// async fn add_example(pool: &PgPool) -> Result<Notebook, NotebookError> {
///     let row = add_note("add_note", "Added a little note so you don't forget", pool).await?;
///
///     assert_eq!("add_note", row.note_name);
///
///     Ok(row)
/// }
/// ```
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

/// Deleting the requested note
/// ### Returns
/// * Errors
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
/// ### Example
/// ```rust,no run
/// async fn delete_example(pool: &PgPool) -> Result<Notebook, NotebookError> {
///     let row = add_note("bad_cat", "Buy new slippers. The old ones were ruined by the cat", pool).await?;
///
///     delete_note(&row.note_name, pool).await?;
///
///     // Should return error because note `bad_cat` is not exist
///     print_note(&row.note_name, pool).await?;
///
///     Ok(row)
/// }
/// ```
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
            let row_note = if let Some(n) = &row.note { n } else { "" };

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

/// Deleting all total notes in notebook
/// ### Returns
/// * Errors
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
/// ### Example
/// ```rust,no run
/// async fn delete_all_example(pool: &PgPool) -> Result<Notebook, NotebookError> {
///     add_note("bad_cat", "Buy new slippers. the old ones were ruined by the cat", pool).await?;
///     add_note("cool_cat", "Don't forget to post a photo of my cool cat", pool).await?;
///     add_note("empty", "", pool).await?;
///
///     delete_all_notes(pool).await?;
///
///     // Should display empty list
///     print_all_notes(pool).await?;
///
///     Ok(row)
/// }
/// ```
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

/// Updating old note to new note
/// ### Returns
/// * Ok
///     * [note of `Notebook` type][Notebook] that was updated
/// * Errors
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
/// ### Example
/// ```rust,no run
/// async fn update_note_example(pool: &PgPool) -> Result<Notebook, NotebookError> {
///     let row = add_note("wrong_note", "Thos is erong nlte", pool).await?;
///     let row_note = if let Some(n) = row.note { n } else { "" };
///
///     assert_eq!("Thos is erong nlte", row_note);
///
///     let upd_row = update_note("wrong_note", "This is NOT wrong note");
///     let upd_row_note = if let Some(n) = upd_row.note { n } else { "" };
///
///      assert_eq!("This is NOT wrong note", upd_row_note);
///
///     Ok(row)
/// }
/// ```
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

/// Updating old notename to new notename
/// ### Returns
/// * Ok
///     * [note of `Notebook` type][Notebook] that was updated
/// * Errors
///     * [`NotebookError::Sqlx`][NotebookError] error from [`sqlx::Error`]
/// ### Example
/// ```rust,no run
/// async fn update_notename_example(pool: &PgPool) -> Result<Notebook, NotebookError> {
///     let row = add_note("wrlng_nptenAme", "", pool).await?;
///
///     assert_eq!("wrlng_nptenAme", row.note_name);
///
///     let upd_row = update_note_name("wrlng_nptenAme", "not_wrong_name");
///
///     assert_eq!("not_wrong_name", upd_row_note);
///
///     Ok(row)
/// }
/// ```
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
