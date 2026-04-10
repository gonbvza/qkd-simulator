use std::sync::{Arc, Condvar, Mutex};

use crate::cli::command::Command;
use crate::cli::methods::{
    create_link_cli, create_node_cli, get_links_cli, get_nodes_cli, start_qkd_cli,
};
use crate::error::Error;
use crate::utility::read_line;

// TODO: Create cli error
pub async fn run_cli() -> Result<(), Error> {
    loop {
        println!("What do you want to do?");
        let command: Command = match read_line().try_into() {
            Ok(cmd) => cmd,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };
        match command {
            Command::CreateNode => create_node_cli().await?,
            Command::CreateLink => create_link_cli().await?,
            Command::Start => start_qkd_cli().await?,
            Command::GetNodes => get_nodes_cli().await?,
            Command::GetLinks => get_links_cli().await?,
            Command::Exit => break,
        }
    }
    Ok(())
}
