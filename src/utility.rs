use std::io;

pub fn verify_args() {
    todo!()
}

pub fn read_line() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    return buffer.trim().to_string();
}

#[macro_export]
macro_rules! get_node_arg {
    ($args:expr, $key:expr) => {{
        match $args.get($key) {
            Some(EventArgs::Node(node)) => node,
            _ => {
                return Err(Error::Sim(SimError::MissingArgument($key.to_string())));
            }
        }
    }};
}

#[macro_export]
macro_rules! get_link_arg {
    ($args:expr, $key:expr) => {{
        match $args.get($key) {
            Some(EventArgs::Link(link)) => link,
            _ => {
                return Err(Error::Sim(SimError::MissingArgument($key.to_string())));
            }
        }
    }};
}

#[macro_export]
macro_rules! get_qubit_ref_arg {
    ($args:expr, $key:expr) => {{
        match $args.get($key) {
            Some(EventArgs::QubitRef(qubit_ref)) => qubit_ref,
            _ => {
                return Err(Error::Sim(SimError::MissingArgument($key.to_string())));
            }
        }
    }};
}
