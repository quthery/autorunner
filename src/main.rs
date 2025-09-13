mod cli;
mod hash;


use clap::{Parser};
use cli::args::CliArgs;


fn main() {
    let args = CliArgs::parse(); 
    let string_path = args.path.as_deref().expect("Path argument is required");
    match hash::calculate::calc(string_path) {
        Ok(hash) =>
        {
            println!("hash: {}",hash)
        },
        Err(e) => 
        {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
    println!("{:?}", args);
}

