use crate::{
    api::{create_link_api, create_node_api},
    database::{
        link::get_all_links,
        nodes::{get_all_nodes, get_node_by_id},
    },
    error::{CliError, Error},
    establish_connection,
    models::links::Link,
    nodes::nodes::Node,
    utility::read_line,
};

pub async fn create_node_cli() -> Result<(), Error> {
    println!("Enter node name:");
    let name = read_line();
    println!("Enter node type:");
    let node_type = read_line();
    create_node_api(name.trim().to_string(), node_type.parse()?).await?;
    Ok(())
}

pub async fn create_link_cli() -> Result<(), Error> {
    println!("Enter source node id:");
    let src_id = read_line().trim().parse::<i32>().unwrap();
    println!("Enter destination node id:");
    let dst_id = read_line().trim().parse::<i32>().unwrap();

    // TODO: Request rest of link attr

    create_link_api(src_id, dst_id).await?;
    Ok(())
}

pub async fn start_qkd_cli() -> Result<(), Error> {
    let mut conn = establish_connection();
    println!("Enter source id:");
    let src_node: Node = get_node_by_id(
        &mut conn,
        read_line().trim().parse::<i32>().map_err(CliError::from)?,
    )?;

    println!("Enter destination id:");
    let dst_node: Node = get_node_by_id(
        &mut conn,
        read_line().trim().parse::<i32>().map_err(CliError::from)?,
    )?;
    // start_qkd(src_node, dst_node).await;
    Ok(())
}

pub async fn get_nodes_cli() -> Result<(), Error> {
    let mut conn = establish_connection();
    let nodes: Vec<Node> = get_all_nodes(&mut conn)?;
    for node in nodes {
        println!("{}", node);
    }
    Ok(())
}

pub async fn get_links_cli() -> Result<(), Error> {
    let mut conn = establish_connection();
    let mut links: Vec<Link> = get_all_links(&mut conn)?;
    for link in links {
        println!("{}", link);
    }
    Ok(())
}
