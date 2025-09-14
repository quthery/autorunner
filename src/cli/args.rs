use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rerunner", about = "A simple process rerunner")] 
pub struct CliArgs {
    #[arg(long, short='p', help = "Path to a file or directory", required=true)]
    pub  path: Option<String>,

    #[arg(long, short = 'c', help = "Command to run", required=true)]
    pub command: String,
}
