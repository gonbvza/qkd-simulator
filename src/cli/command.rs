use crate::error::CliError;

#[derive(Debug)]
pub enum Command {
    CreateNode,
    CreateLink,
    GetNodes,
    GetLinks,
    Start,
    Exit,
}

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
