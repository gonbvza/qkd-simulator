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
