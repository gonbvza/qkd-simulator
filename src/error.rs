use thiserror::Error;

// errors/link.rs
#[derive(Error, Debug)]
pub enum LinkError {
    #[error("Nodes {0} and {1} do not exist")]
    NonExistingNodes(i32, i32),
    #[error("Link between nodes already exists")]
    DuplicateLink,
    #[error("Link capacity exceeded")]
    CapacityExceeded,
    #[error("Link does not exist between {0} and {1}")]
    NotExistingLink(i32, i32),
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
}

// errors/link.rs
#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Node with id {0} already exists")]
    AlreadyExists(String),
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    #[error("Node with id {0} is already in use")]
    NodeInUse(i32),
}

// errors/cli.rs
#[derive(Error, Debug)]
pub enum CliError {
    #[error("Input by the user was not valid")]
    NotValidInput(String),
    #[error("Not valid integer: {0}")]
    NoValidInteger(#[from] std::num::ParseIntError),
}

// errors/cli.rs
#[derive(Error, Debug)]
pub enum SimError {
    #[error("Input by the user was not valid")]
    NotValidInput(String),
}

// TODO: Change this to macro
pub fn map_db_error(node_id: String, e: diesel::result::Error) -> NodeError {
    match e {
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        ) => NodeError::AlreadyExists(node_id),
        other => NodeError::Database(other),
    }
}

#[derive(Error, Debug)]
pub enum Error {
    // Link errors
    #[error("{0}")]
    Link(#[from] LinkError),
    // Node errors
    #[error("{0}")]
    Node(#[from] NodeError),
    // Cli errors
    #[error("{0}")]
    Cli(#[from] CliError),
}
