use thiserror::Error;

// graph errors
#[derive(Error, Debug)]
pub enum GraphError {
    // Node errors
    #[error("{0}")]
    Node(#[from] NodeError),
    // Link errors
    #[error("{0}")]
    Link(#[from] LinkError),
    #[error("No common epr between {0} and {1}")]
    NoCommonEpr(i32, i32),
}

// link errors
#[derive(Error, Debug)]
pub enum LinkError {
    #[error("Nodes {0} and {1} do not exist")]
    NonExistingNodes(i32, i32),
    #[error("Nodes {0} is missing")]
    MissingNode(i32),
    #[error("Link between nodes already exists")]
    DuplicateLink,
    #[error("Link capacity exceeded")]
    CapacityExceeded,
    #[error("Link does not exist between {0} and {1}")]
    NotExistingLink(i32, i32),
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
}

// link errors
#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Node with id {0} already exists")]
    AlreadyExists(String),
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    #[error("Node is already in use")]
    NodeInUse(),
    #[error("Not valid node kind: {0}")]
    NotValidKind(String),
    #[error("Error while creating the detector")]
    DetectorErro(#[from] DetectorError),
}

// cli errors
#[derive(Error, Debug)]
pub enum CliError {
    #[error("Input by the user was not valid")]
    NotValidInput(String),
    #[error("Command {0} is not valid")]
    NotValidCommand(String),
    #[error("Not valid integer: {0}")]
    NoValidInteger(#[from] std::num::ParseIntError),
}

// simulation errors
#[derive(Error, Debug)]
pub enum SimError {
    #[error("Input by the user was not valid")]
    NotValidInput(String),
    #[error("Missing arg argument: {0}")]
    MissingArgument(String),
}

// entangled_pair errors
#[derive(Error, Debug)]
pub enum PairError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    #[error("Database error: {0}")]
    Measurement(#[from] MeasurementError),
    #[error("Node error: {0}")]
    Node(#[from] NodeError),
    #[error("Entangled pair {0} not present")]
    PairNotFound(i32),
    #[error("Measurement missing")]
    NotMeasured(),
}

// process errors
#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    #[error("Process missing")]
    NotFound(),
}

// detector errors
#[derive(Error, Debug)]
pub enum DetectorError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    #[error("Current detector {0} is busy cooling down")]
    CoolingDown(i32),
    #[error("Detector for node {0} not found")]
    NotFound(i32),
}

// measurement errors
#[derive(Error, Debug)]
pub enum MeasurementError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
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
    // Graph errors
    #[error("{0}")]
    Graph(#[from] GraphError),
    // Simulation errors
    #[error("{0}")]
    Sim(#[from] SimError),
    // Pair errors
    #[error("{0}")]
    Pair(#[from] PairError),
    //Process Error
    #[error("{0}")]
    Process(#[from] ProcessError),
    //Detector error
    #[error("{0}")]
    Detector(#[from] DetectorError),
    //Measurement error
    #[error("{0}")]
    Measurement(#[from] MeasurementError),

    // Event loop error
    #[error("Function {0} does not exist")]
    NonExistantFunction(String),
}
