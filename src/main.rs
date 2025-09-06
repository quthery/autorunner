use sha2::{Digest, Sha256};
use std::fs::File;
use std::path::Path;
use std::io::{self, Read};
use walkdir::WalkDir;
use std::process::Command;
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
    let path = Path::new(dir);
    let mut prev_hash = String::new();


    if !path.is_dir() {
        eprintln!("Error: {} is not a directory", dir);
        std::process::exit(1);
    }

    println!("Watching directory: {}", dir);
    println!("Command to execute: {}", command);
    
    loop {
        match compute_folder_sha256(dir) {
            Ok(hash) => {
                if prev_hash != hash {
                    println!("\n--- Changes detected, running command ---");

                    let command_parts: Vec<&str> = command.split_whitespace().collect();
                    let program = command_parts[0];
                    let args: Vec<&str> = command_parts[1..].to_vec();

                    match Command::new(program)
                        .args(&args)
                        .output() {
                            Ok(output) => {
                                if !output.stdout.is_empty() {
                                    println!("\n=== Command Output ===");
                                    println!("{}", String::from_utf8_lossy(&output.stdout));
                                }
                                if !output.stderr.is_empty() {
                                    println!("\n=== Error Output ===");
                                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                                }
                            },
                            Err(e) => {
                                eprintln!("\nFailed to execute command: {}", e);
                            }
                    }
                    
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