use crate::error::CliError;

/// CLI commands accepted by the interactive prompt.
#[derive(Debug)]
pub enum Command {
    /// Create a node.
    CreateNode,
    /// Create a link between two nodes.
    CreateLink,
    /// List known nodes.
    GetNodes,
    /// List known links.
    GetLinks,
    /// Start a QKD session.
    Start,
    /// Stop the interactive CLI.
    Exit,
}

/// Converts raw user input into a typed [`Command`].
impl TryFrom<String> for Command {
    type Error = CliError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let s = s.trim().to_lowercase();
        match s.as_str() {
            "create_node" => Ok(Command::CreateNode),
            "create_link" => Ok(Command::CreateLink),
            "get_nodes" => Ok(Command::GetNodes),
            "get_links" => Ok(Command::GetLinks),
            "start" => Ok(Command::Start),
            "exit" => Ok(Command::Exit),
            _ => Err(CliError::NotValidCommand(s)),
        }
    }
}
