pub mod commands;
pub use commands::execute_commands::NoteCommand;
pub use commands::{
    add_note, delete_all_notes, delete_note, print_all_data, update_note, update_notename,
};

pub mod errors;
pub use errors::DataBaseError;
