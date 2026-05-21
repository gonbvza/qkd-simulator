//! Interactive CLI runner.
//!
//! This file hosts the top-level input loop and dispatches each parsed command
//! to the corresponding handler in `methods`.
use crate::cli::command::Command;
use crate::cli::methods::{
    create_link_cli, create_node_cli, get_links_cli, get_nodes_cli, start_qkd_cli,
};
use crate::core::event_loop::EventLoopHandler;
use crate::error::Error;
use crate::utility::read_line;

/// Runs the interactive CLI loop until `exit` is entered.
///
/// The loop is intentionally resilient to command parsing failures:
/// invalid input is reported and the prompt is shown again.
pub async fn run_cli(handle: EventLoopHandler) -> Result<(), Error> {
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
            Command::CreateNode => create_node_cli(&handle).await?,
            Command::CreateLink => create_link_cli(&handle).await?,
            Command::Start => start_qkd_cli(&handle).await?,
            Command::GetNodes => get_nodes_cli().await?,
            Command::GetLinks => get_links_cli().await?,
            Command::Exit => break,
        }
    }
    Ok(())
}
