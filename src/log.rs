use serde::{Deserialize, Serialize};

use crate::{Term, Index};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogEntry {
    cmd: Command,
    term: Term,
    index: Index,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Command {
    Set {
        key: String,
        value: String,
    },
    Update {
        key: String,
        value: String,
    },
    Delete {
        key: String,
    }
}