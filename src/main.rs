mod cli;
mod hash;
mod runner;


use std::process::Child;

use clap::{Parser};
use cli::args::CliArgs;

use crate::runner::process;


fn main() {
    let args = CliArgs::parse(); 
    let string_path = args.path.as_deref().expect("Path argument is required");
    let mut prev_hash = String::new();
    let mut child: Child = process::spawn(&args.command)
                    .expect("Error on executing command");
                
    loop {
        match hash::calculate::calc(string_path) {
                Ok(hash) =>
                {
                   if hash != prev_hash {
                        match child.kill() {
                            Ok(_) =>
                            {
                                child = process::spawn(&args.command).expect("pizdec")
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




