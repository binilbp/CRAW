use clap::{Parser, Subcommand};
use craw::run_setup;
use std::error::Error;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Pass content to LLM
    content: Option<String>,

    /// Pass custom prompt
    #[arg(short, long)]
    prompt: Option<String>,

    /// Pass file to LLM
    #[arg(short, long)]
    file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start here. Setup CRAW
    Setup,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut cfg = confy::load("craw", "craw-config")?;

    match args.command {
        Some(Commands::Setup) => run_setup(&mut cfg)?,
        None => print_about(),
    }
    Ok(())
}

//detailed custom about message when call CRAW without args
fn print_about() {
    println!(
        "\
\n------
CRAW 
------
\nYet another cli AI wrapper in RUST

This tool was created with intention to learn RUST while trying to
solve my annoyance of opening a browser or learning a new already
available better tool to access LLM from a terminal.
Use nerdfont for better experience !
\n-> `craw setup` to get started
-> `craw help` for more usage info
\nFeel free to suggest changes or express your issues
ps: the naming was hard, Cli Rust Ai Wrapper ;)
"
    )
}
