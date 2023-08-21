use std::process;
use clap::Parser;

mod app;
mod time;
use app::Cli;

fn main() {
    let args = Cli::parse();
    
    if let Err(e) = app::run(args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
