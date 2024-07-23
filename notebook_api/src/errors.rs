//! Errors returned from the [`notebook API`][crate]

#[derive(thiserror::Error, Debug)]
/// Conatins all errors returned from the notebook API
pub enum NotebookError {
    /// The note-name is already taken; try use another note-name
    #[error("The note-name `{notename}` is already taken; try use another note-name")]
    AlreadyTaken { notename: String },

    /// Data base enivroment variable for notebook not specifed;
    /// Try use `export DATABASE_URL=postgres://username:password@localhost/db` before start programm
    #[error(
        "Data base enivroment variable for notebook not specifed;
Try use `export DATABASE_URL=postgres://username:password@localhost/db` before start programm"
    )]
    DatabaseNotSpecifed,

    /// All errors from [`sqlx::Error`]
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    /// All errors from [`env::VarError`][std::env::VarError]
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}
