use std::fs;
use std::path::Path;
use std::io::{Error, ErrorKind};
use walkdir::WalkDir;


pub fn calc(path_str: &str) -> Result<String, Error> {
    let path = Path::new(path_str);
    if !path.exists() {
        return Err(Error::new(ErrorKind::NotFound, "Path not found"));
    }
    if path.is_dir() {
        for_folder(&path)
    } else {
        for_file(path)
    }

    
}


fn for_file(path: &Path) -> Result<String, Error> {
        let bytes = fs::read(path)?;
        let hash = sha256::digest(&bytes);
        Ok(hash)
}



fn for_folder(path: &Path) -> Result<String, Error> {
    let mut files_hashes: Vec<String> = Vec::new();
    let mut entries: Vec<String> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| match e {
            Ok(entry) => Some(entry),
            Err(e) => {
                eprintln!("Warning: skipping unreadable entry: {}", e);
                None
            }
        })
        .filter(|e| e.path().is_file())
        .filter_map(|e| match e.path().to_str() {
            Some(s) => Some(s.to_string()),
            None => {
                eprintln!("Warning: skipping entry with non-UTF-8 path: {:?}", e.path());
                None
            }
        })
        .collect();
    entries.sort();
     
    for entry in entries.iter(){
        let entry_path = Path::new(entry);
        let file_content = fs::read(entry_path)?;
        let hash = sha256::digest(&file_content[..]);
        files_hashes.push(hash);
    }

    let hash = sha256::digest(files_hashes.join("\n")); 

    Ok(hash)
}
        
