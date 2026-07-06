use clap::{Parser, Subcommand};
use craw::{APP_NAME, CONFIG_FILE_NAME, config::Config, prompt_agent, run_reset, run_setup};
use std::{error::Error, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Simple prompt and reply. Eg: craw -p "hello"
    #[arg(short, long)]
    prompt: Option<String>,

    /// Pass file to LLM
    #[arg(short, long)]
    file: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Print info message
    About,
    /// Setup craw, start here
    Setup,
    /// Reset craw
    Reset,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut cfg: Config = confy::load(APP_NAME, CONFIG_FILE_NAME)?;
    let context: Option<String> = None;
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
            Commands::Reset => {
                run_reset(&cfg)?;
                return Ok(());
            }
        }
    }

    //first run or api setup not done
    if cfg.api_service.is_none() {
        print_about();
    } else {
        run_app(&args, &cfg, context.as_deref()).await?;
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

async fn run_app(args: &Args, cfg: &Config, context: Option<&str>) -> Result<(), Box<dyn Error>> {
    if let Some(prompt) = &args.prompt {
        let reply = prompt_agent(cfg, prompt, context).await?;
        println!(": {reply}");
    } else {
        print_about();
    }
    Ok(())
}
