use crate::api::{create_link_cli, create_node_cli, start_qkd};
use crate::database::nodes::get_node_by_id;
use crate::error::{CliError, Error};
use crate::establish_connection;
use crate::nodes::nodes::Node;
use std::io;

// TODO: Create cli error
pub async fn run_cli() -> Result<(), Error> {
    let mut conn = establish_connection();
    loop {
        println!("What do you want to do?");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line");

        match buffer.trim() {
            "Create" => {
                println!("Enter node name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read name");
                println!("Enter node type:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read name");

                create_node_cli(name.trim().to_string(), name.parse()?).await?;
            }
            "Link" => {
                println!("Enter nodea id:");
                let mut nodea = String::new();
                io::stdin()
                    .read_line(&mut nodea)
                    .expect("Failed to read name");
                println!("Enter nodeb id:");
                let mut nodeb = String::new();
                io::stdin()
                    .read_line(&mut nodeb)
                    .expect("Failed to read name");
                let a = nodea.trim().parse::<i32>().unwrap();
                let b = nodeb.trim().parse::<i32>().unwrap();

                create_link_cli(a, b).await?;
            }
            "Start" => {
                println!("Enter sender id:");
                let mut src_id = String::new();
                io::stdin()
                    .read_line(&mut src_id)
                    .expect("Failed to read name");
                let src_node: Node =
                    get_node_by_id(&mut conn, src_id.parse::<i32>().map_err(CliError::from)?)?;
                println!("Enter receier id:");
                let mut dst_id = String::new();
                io::stdin()
                    .read_line(&mut dst_id)
                    .expect("Failed to read name");
                let dst_node: Node =
                    get_node_by_id(&mut conn, dst_id.parse::<i32>().map_err(CliError::from)?)?;
                start_qkd(src_node, dst_node).await;
            }

            "Exit" => break,
            _ => println!("Unknown command"),
        }
    }

    Ok(())
}
