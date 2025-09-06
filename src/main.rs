use sha2::{Digest, Sha256};
use std::fs::File;
use std::path::Path;
use std::io::{self, Read};
use walkdir::WalkDir;
use pico_args::Arguments;
use std::process::{Command};

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
        // println!("{}", entry);
        hasher.update(entry.as_bytes());
        let file_hash = compute_file_sha256(&entry)?;
        hasher.update(&file_hash);

    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

fn main()  {
    let term = console::Term::stdout();
    let mut args = Arguments::from_env();
    let dir: String = args.value_from_str("--dir").expect("Enter directory path in --dir <path> or -d <path>"); 
    let command: String = args.value_from_str("--command").expect("Enter command to run in --command or -c <command>");
    let path = Path::new(&dir);
    let mut hash: Vec<u8> = Vec::new();
    let mut prev_hash = String::new();
    // FIXME
    if path.is_file() {
        loop {
            hash = compute_file_sha256(&dir).expect("hello");
            let hash_string = String::from_utf8(hash).expect("An error occurred");
            if  hash_string != prev_hash {
               Command::new("zsh").arg(&command);
            }
            prev_hash = hash_string;
        }    
    }
    if path.is_dir() {
    println!("its folder\n\n");
    loop {
        match compute_folder_sha256(&dir) {
            Ok(hash) => {
                if prev_hash != hash {
                    term.clear_screen().expect("An error окуред");
                    let output = Command::new("zsh")
                        .arg("-c")
                        .arg(&command)
                        .output()
                        .map_err(|e| eprintln!("Ошибка выполнения команды: {}", e)).expect("An occured error");
                    
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                    prev_hash = hash;
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
                

            }
            Err(e) => {
                eprintln!("Ошибка: {}", e);
                break;
            }
        }
    }
}
}