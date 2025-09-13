use std::process::{Command, Child, Stdio};
use std::thread;
use std::io::{self,BufRead, BufReader};

pub fn spawn(command: &str) -> io::Result<(Child, thread::JoinHandle<()>)> {
    let mut child = Command::new("sh")
    .arg("-c")
    .arg(command)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Не удалось запустить команду '{}': {}", command, e)))?;

    let stdout = child.stdout.take().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Не удалось получить stdout"))?;
    let stderr = child.stderr.take().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Не удалось получить stderr"))?;

    let handle = thread::spawn(move || {
        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);

        for line in stdout_reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(e) => eprintln!("Ошибка чтения stdout: {}", e),
            }
        }

        for line in stderr_reader.lines() {
            match line {
                Ok(line) => eprintln!("{}", line),
                Err(e) => eprintln!("Ошибка чтения stderr: {}", e),
            }
        }
    });

    Ok((child, handle))
}   
