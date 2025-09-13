use std::process::{Command, Child};
use std::io;

pub fn spawn(command: &str) -> io::Result<Child> {
    return Command::new("sh")
    .arg("-c")
    .arg(command)
    .spawn()
    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Не удалось запустить команду: {}", e)));
}