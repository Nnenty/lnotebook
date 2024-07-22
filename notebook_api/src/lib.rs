//! # Notebook_api
//! `Notebook_api` is an API for creating notebooks that store notes in a database.

pub mod commands;
pub use commands::execute_commands::NoteCommand;

pub mod errors;
pub use errors::NotebookError;

use std::env;

pub async fn get_db_url() -> Result<String, NotebookError> {
    let ret_db = match env::var("DATABASE_URL") {
        Ok(ok_db) => ok_db,

        Err(db_err) => {
            if db_err == std::env::VarError::NotPresent {
                return Err(NotebookError::DataBaseNotSpecifed);
            }

            return Err(NotebookError::VarError(db_err));
        }
    };

    Ok(ret_db)
}
