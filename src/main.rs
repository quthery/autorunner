mod cli;
mod hash;
mod runner;

use clap::Parser;
use cli::args::CliArgs;
use std::io::{self};
use crate::runner::process;
use std::time::Duration;
use std::thread;

fn main() -> io::Result<()> {
    let args = CliArgs::parse(); 
    let string_path = args.path.as_deref().expect("Path argument is required");
    let hash = hash::calculate::calc(string_path).expect("error on getting hash");
    let mut prev_hash = hash;
    
    let (mut child, mut output_handle, mut should_stop) = process::spawn(&args.command)
        .expect("Error on executing command");

    loop {
        match hash::calculate::calc(string_path) {
            Ok(hash) => {
                if hash != prev_hash {
                    println!("Change detected, restarting process....\n");

                    //  флаг остановки
                    {
                        let mut stop = should_stop.lock().unwrap();
                        *stop = true;
                    }


                    // Убиваем процесс
                    if let Err(e) = child.kill() {

                        eprintln!("Error killing process: {}", e);
                        std::process::exit(-1);
                    }

                    // Ждем завершения 
                    if let Err(e) = child.wait() {
                        eprintln!("Error waiting for process: {}", e);
                    }
  
                    // Ждем завершения  
                    if let Err(e) = output_handle.join() {
                        eprintln!("Error joining output thread: {:?}", e);
                    } 
                    // новый процесс
                    match process::spawn(&args.command) {
                        Ok((new_child, new_output_handle, new_should_stop)) => {
                            child = new_child;
                            output_handle = new_output_handle;
                            should_stop = new_should_stop;
                            println!("New process spawned successfully");
                        }
                        Err(e) => {
                            eprintln!("Error spawning new process: {}", e);
                            std::process::exit(-1);
                        }
                    }

                    prev_hash = hash;
                }
            },
            Err(e) => {
                eprintln!("Error calculating hash: {}", e);
                std::process::exit(1);
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}