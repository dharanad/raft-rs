use crate::{log::LogEntry, Index, Term};
struct ServerState {
    // Persistent State
    current_term: Term,
    voted_for: u32,
    log: Vec<LogEntry>,

    // Volatile or In Memory
    commit_index: Index,
    last_applied: Index,

    // For leaders
    next_index: Vec<Index>,
    match_index: Vec<Index>
}

#[derive(Debug, Clone)]
enum ServerType {
    Leader,
    Follower,
    Candidate
}