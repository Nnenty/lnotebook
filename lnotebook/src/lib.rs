//! # LNotebook
//! `LNotebook` is a simple asynchronous API for creating notebooks that store notes in a database.

pub mod commands;
pub use commands::execute_commands::NoteCommand;
pub mod errors;
pub use errors::NotebookError;

use std::env;

/// Gets database URL drom enivroment variable `DATABASE_URL`.
/// ### Returns
/// * Ok
///     * Returns the database URL as a `String`
/// * Errors
///     * Returns [`NotebookError::DatabaseNotSpecifed`] error if you didn't specify the database in the
/// enivroment variable `DATABASE_URL`
///     * Returns [`NotebookError::VarError`] error from [`env::VarError`]
/// if any other [`env::VarError`] occurs
/// ### Example
/// ```
/// async fn get_url_example() -> Result<(), NotebookError> {
///     // Works only if you specidied env `DATABASE_URL`
///     let db_url = get_db_url().await?;
///     
///     assert_eq(db_url, "postgres://your_usname:your_password@localhost/your_db");
///
///     Ok(())
/// }
/// ```
pub async fn get_db_url() -> Result<String, NotebookError> {
    let ret_db = match env::var("DATABASE_URL") {
        Ok(ok_db) => ok_db,

        Err(db_err) => {
            if db_err == env::VarError::NotPresent {
                return Err(NotebookError::DatabaseNotSpecifed);
            }

            return Err(NotebookError::VarError(db_err));
        }
    };

    Ok(ret_db)
}
