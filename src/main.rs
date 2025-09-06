use sha2::{Digest, Sha256};
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, BufReader, Read};
use walkdir::WalkDir;
use std::process::{Command, Stdio};
use std::env;

fn compute_file_sha256(file_path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    Ok(hasher.finalize().to_vec())
}

fn compute_folder_sha256(folder_path: &str) -> io::Result<String> {
    let mut hasher = Sha256::new();
    
    let mut entries: Vec<_> = WalkDir::new(folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect();
    entries.sort();

    for entry in entries {
        hasher.update(entry.as_bytes());
        let file_hash = compute_file_sha256(&entry)?;
        hasher.update(&file_hash);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <directory_path> <command>", args[0]);
        std::process::exit(1);
    }

    let dir = &args[1];
    let command = &args[2];
    println!("Watching directory: {}", dir);
    println!("Command to execute: {}", command);
    
    let path = Path::new(dir);
    let mut prev_hash = String::new();

    if !path.is_dir() {
        eprintln!("Error: {} is not a directory", dir);
        std::process::exit(1);
    }
    
    loop {
        match compute_folder_sha256(dir) {
            Ok(hash) => {
                if prev_hash != hash {
                    println!("\n--- Changes detected, running command ---");

                    // Настраиваем команду с перенаправлением stdout и stderr
                    let mut cmd = Command::new("sh")
                        .arg("-c")
                        .arg(command)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()
                        .expect("Failed to execute command");

                    // Получаем stdout и stderr
                    let stdout = cmd.stdout.take().expect("Failed to capture stdout");
                    let stderr = cmd.stderr.take().expect("Failed to capture stderr");

                    // Создаем BufReader для чтения вывода построчно
                    let stdout_reader = BufReader::new(stdout);
                    let stderr_reader = BufReader::new(stderr);

                    // Запускаем поток для stdout
                    std::thread::spawn(move || {
                        for line in stdout_reader.lines() {
                            match line {
                                Ok(line) => println!("{}", line),
                                Err(e) => eprintln!("Error reading stdout: {}", e),
                            }
                        }
                    });

                    // Запускаем поток для stderr
                    std::thread::spawn(move || {
                        for line in stderr_reader.lines() {
                            match line {
                                Ok(line) => eprintln!("{}", line),
                                Err(e) => eprintln!("Error reading stderr: {}", e),
                            }
                        }
                    });

                    // Ждем завершения команды
                    let status = cmd.wait().expect("Failed to wait on command");
                    println!("Command finished with status: {:?}", status);
                    
                    prev_hash = hash;
                }
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
            Err(e) => {
                eprintln!("Error watching directory: {}", e);
                break;
            }
        }
    }
}