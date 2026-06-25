use crate::{
    api::{create_link_api, create_node_api, get_links_api, get_nodes_api, start_qkd},
    core::event_loop::EventLoopHandler,
    error::{CliError, Error},
    utility::read_line,
};

/// Prompts for node fields and forwards node creation to the API layer.
pub async fn create_node_cli(handle: &EventLoopHandler) -> Result<(), Error> {
    println!("Enter node name:");
    let name = read_line();
    println!("Enter node type:");
    let node_type = read_line();
    create_node_api(name.trim().to_string(), node_type.parse()?, handle).await?;
    Ok(())
}

/// Prompts for link fields and forwards link creation to the API layer.
pub async fn create_link_cli(handle: &EventLoopHandler) -> Result<(), Error> {
    println!("Enter source node id:");
    let src_id = read_line().trim().parse::<i32>().map_err(CliError::from)?;
    println!("Enter destination node id:");
    let dst_id = read_line().trim().parse::<i32>().map_err(CliError::from)?;
    println!("Enter distance in meters:");
    let distance = read_line().trim().parse::<i64>().map_err(CliError::from)?;
    println!("Is secure (true or false):");
    let is_secure: bool = read_line().trim().parse::<bool>().map_err(CliError::from)?;
    create_link_api(src_id, dst_id, distance, is_secure, handle).await?;
    Ok(())
}

/// Prompts for QKD endpoints and requests session startup through the API.
pub async fn start_qkd_cli(handle: &EventLoopHandler) -> Result<(), Error> {
    println!("Enter source id:");
    let src_node_id = read_line().trim().parse::<i32>().map_err(CliError::from)?;
    println!("Enter destination id:");
    let dst_node_id = read_line().trim().parse::<i32>().map_err(CliError::from)?;
    start_qkd(src_node_id, dst_node_id, handle).await?;
    Ok(())
}

/// Fetches all nodes through the API and prints them for the operator.
pub async fn get_nodes_cli() -> Result<(), Error> {
    let nodes = get_nodes_api().await?;
    if nodes.is_empty() {
        println!("No nodes were found");
        return Ok(());
    }
    for node in nodes {
        println!("{}", node);
    }
    Ok(())
}

/// Fetches all links through the API and prints them for the operator.
pub async fn get_links_cli() -> Result<(), Error> {
    let links = get_links_api().await?;
    if links.is_empty() {
        println!("No links were found");
        return Ok(());
    }
    for link in links {
        println!("{}", link);
    }
    Ok(())
}
