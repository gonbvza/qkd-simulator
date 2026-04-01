use crate::api::{create_link_cli, create_node_cli, start_qkd};
use crate::error::{CliError, Error};
use std::io;

// TODO: Create cli error
pub async fn run_cli() -> Result<(), Error> {
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
                let mut sender_id = String::new();
                io::stdin()
                    .read_line(&mut sender_id)
                    .expect("Failed to read name");
                println!("Enter receier id:");
                let mut receiver_id = String::new();
                io::stdin()
                    .read_line(&mut receiver_id)
                    .expect("Failed to read name");
                println!("Enter epr id:");
                let mut epr_id = String::new();
                io::stdin()
                    .read_line(&mut epr_id)
                    .expect("Failed to read name");
                start_qkd(
                    sender_id.parse::<i32>().map_err(CliError::from)?,
                    receiver_id.parse::<i32>().map_err(CliError::from)?,
                    epr_id.parse::<i32>().map_err(CliError::from)?,
                );
            }

            "Exit" => break,
            _ => println!("Unknown command"),
        }
    }

    Ok(())
}
