use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinkError {
    #[error("The nodes do not exist")]
    NonExistingNodes,
}

#[derive(Error, Debug)]
pub enum Error {
    // Link errors
    #[error("{0}")]
    Link(#[from] LinkError),
}
