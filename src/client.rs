use std::fmt::Error;

use crate::{log::LogEntry, Index, NodeId, Term};

pub struct RpcClient {

}

impl RpcClient {
    fn append_entries(req: AppendEntriesRequest) -> Result<AppendEntriesResponse, Error> {
        unimplemented!()
    }

    fn request_vote(req: RequestVoteRequest) -> Result<RequestVoteResponse, Error> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
struct AppendEntriesRequest {
    term: Term,
    leader_id: NodeId,
    prev_log_index: Index,
    prev_log_term: Term,
    entires: Vec<LogEntry>
}

#[derive(Debug, Clone)]
struct AppendEntriesResponse {
    term: Term,
    status: bool
}

#[derive(Debug, Clone)]
struct RequestVoteRequest {
    term: Term,
    candidate_id: NodeId,
    last_log_index: Index,
    last_log_term: Term,
}

#[derive(Debug, Clone)]
struct RequestVoteResponse {
    term: Term,
    vote_granted: bool
}