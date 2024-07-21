#[derive(thiserror::Error, Debug)]
pub enum NotebookError {
    #[error("The note-name `{notename}` is already taken; try use another note-name")]
    AlreadyTaken { notename: String },

    #[error(
        "\
        Data base enivroment variable not specifed;\
        \nUse `DATABASE_URL=postgres://username:password@localhost/db cargo run `notebook command`\
        "
    )]
    DataBaseNotSpecifed,

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    VarError(#[from] std::env::VarError),
}
