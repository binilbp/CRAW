use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Pass content to LLM
    content: Option<String>,

    /// Pass custom prompt
    #[arg(short, long)]
    prompt: Option<String>,

    /// Pass file content to LLM
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.content {
        Some(content) => println!("{}", content),
        None => empty_call(),
    }
}

fn empty_call() {
    println!(
        "\
\n------
CRAW 
------
\nYet another cli AI wrapper in RUST

This tool was created with intention to learn RUST while trying to
solve my annoyance of opening a browser or learning a new already
available better tool to access LLM from a terminal.
Use nerdfont for better experience.
\n`craw --help` for more usage info
\nFeel free to suggest changes or express your issues
ps: the naming was hard, Cli Rust Ai Wrapper ;)
"
    )
}
