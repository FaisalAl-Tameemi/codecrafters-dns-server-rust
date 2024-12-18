use thiserror::Error;

#[derive(Error, Debug)]
pub enum DnsError {
    #[error("Invalid query")]
    InvalidQuery,
    #[error("Invalid response")]
    InvalidResponse,
    #[error("Invalid message")]
    InvalidMessage,
}
