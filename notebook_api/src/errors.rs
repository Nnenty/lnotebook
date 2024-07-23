#[derive(thiserror::Error, Debug)]
pub enum NotebookError {
    #[error("The note-name `{notename}` is already taken; try use another note-name")]
    AlreadyTaken { notename: String },

    #[error(
        "Data base enivroment variable for notebook not specifed;
Use `DATABASE_URL=postgres://username:password@localhost/db cargo run `notebook-command`"
    )]
    DatabaseNotSpecifed,

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}
