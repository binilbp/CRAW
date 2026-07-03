use clap::{Parser, Subcommand};
use craw::{config::Config, prompt_agent, run_setup};
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Pass your prompt
    prompt: Option<String>,

    /// Pass context to LLM
    context: Option<String>,

    /// Pass file to LLM
    #[arg(short, long)]
    file: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start here. Setup CRAW
    Setup,
    About,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut cfg: Config = confy::load("craw", "craw-config")?;

    // println!("{:?}", &args);
    // println!("{:?}", &cfg);

    if let Some(sub_command) = args.command {
        match sub_command {
            Commands::Setup => {
                run_setup(&mut cfg)?;
                return Ok(());
            }
            Commands::About => {
                print_about();
                return Ok(());
            }
        }
    }

    //first run or api setup not done
    if cfg.api_service.is_none() {
        print_about();
    } else {
        run_app(&args, &cfg).await?;
    }
    Ok(())
}

//detailed custom about message when call CRAW without args
fn print_about() {
    println!(
        r#"
CRAW   Yet another Cli Rust AI Wrapper

This tool was created with intention to learn RUST while trying to
solve my annoyance of opening a browser or learning a new already
available better tool to access LLM from a terminal.
Use nerdfont for better experience !

-> `craw setup` to get started
-> `craw "your prompt"` to prompt the llm
-> `craw help` for more usage info

Feel free to suggest changes or express your issues

ps: the naming took like 5 mins X)
"#
    )
}

async fn run_app(args: &Args, cfg: &Config) -> Result<(), Box<dyn Error>> {
    if args.context.is_none() && args.prompt.is_some() {
        let prompt = args.prompt.as_ref().unwrap();
        let reply = prompt_agent(prompt, cfg).await?;
        println!(": {reply}");
    } else {
        print_about();
    }
    Ok(())
}
