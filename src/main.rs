use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand)]
enum Commands {
    Test { list: bool },
    Check { name: String },
}
#[derive(Parser)]
#[command(
    author = "ilya roger. <iliailia@me.com>", 
    version = "0.1.0", 
    about = "Simplify migrating your config files from nvim, tmxu and more onto new machines", 
    long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "File")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("value for name: {name}")
    }

    match cli.debug {
        1 => println!("occured once"),
        _ => println!("occured too many times"),
    }
}
