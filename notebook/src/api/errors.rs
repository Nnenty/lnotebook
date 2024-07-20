#[derive(thiserror::Error, Debug)]
pub enum DataBaseError {
    #[error("The note-name `{notename}` is already taken; try use another note-name")]
    AlreadyTaken { notename: String },

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}
