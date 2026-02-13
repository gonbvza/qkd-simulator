use crate::api::create_node_cli;
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
            "Exit" => break,
            _ => println!("Unknown command"),
        }
    }
}
