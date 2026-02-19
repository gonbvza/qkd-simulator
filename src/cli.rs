use crate::api::{create_link_cli, create_node_cli};
use std::io;

pub async fn run_cli() {
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
                create_node_cli(name.trim().to_string()).await;
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

                create_link_cli(a, b).await;
            }

            "Exit" => break,
            _ => println!("Unknown command"),
        }
    }
}
