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
        match for_folder(&path) {
            Ok(hash) =>{
                Ok(hash)
            }
            Err(e) =>
            {
                Err(e)
            }
        }
    } else {
        match for_file(path) {
           Ok(hash) => {
            Ok(hash)
           }
           Err(e) => {
            Err(e)
           }
        } 
    }

    
}


fn for_file(path: &Path) -> Result<String, Error> {
        let bytes = fs::read(path)?;
        let hash = sha256::digest(&bytes);
        Ok(hash)
}



fn for_folder(path: &Path) -> Result<String, Error> {
    let mut files_hashes: Vec<String> = Vec::new();
    let entries: Vec<String> = WalkDir::new(path) // Начинаем с указанной директории
        .into_iter()
        .filter_map(|e| e.ok()) // Пропускаем ошибки
        .filter(|e| e.path().is_file()) // Только файлы
        .filter_map(|e| e.path().to_str().map(String::from)) // Преобразуем путь в String
        .collect(); // Собираем в Vec<String>
     
    for entry in entries.iter(){
        let entry_path = Path::new(entry);
        let file_conent = fs::read(entry_path).unwrap();
        let hash = sha256::digest(String::from_utf8_lossy(&file_conent).into_owned());
        files_hashes.push(entry.clone());
        files_hashes.push(hash);
    }

    let hash = sha256::digest(files_hashes.join("\n")); 

    Ok(hash)
}
        
