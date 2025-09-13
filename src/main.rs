mod cli;
mod hash;
mod runner;


use clap::{Parser};
use cli::args::CliArgs;
use std::io;
use crate::runner::process;


fn main() {
    let args = CliArgs::parse(); 
    let string_path = args.path.as_deref().expect("Path argument is required");
    let mut prev_hash = String::new();
    let (mut child, mut output_handle) = process::spawn(&args.command)
                    .expect("Error on executing command");
    loop {
        
        
        match hash::calculate::calc(string_path) {
                Ok(hash) =>
                {
                   if hash != prev_hash {
                    
                        match child.kill() {
                            Ok(_) =>
                            {
                                (child, output_handle) = process::spawn(&args.command).expect("pizdec");
                                output_handle.join().map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Ошибка потока вывода: {:?}", e)));
                            }
                            Err(e) =>
                            {
                                eprintln!("error while kill process: {}", e);
                                std::process::exit(-1);
                            }
                        }                    
                   } 
                   prev_hash = hash
                },
                Err(e) => 
                {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
    } 
    


}




