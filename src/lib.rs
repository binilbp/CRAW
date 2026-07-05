pub mod config;
mod keyring;

use rig::client::CompletionClient;
use rig::completion::Prompt;
use rig::providers::groq;
use std::{error::Error, io};

use crate::config::Config;

pub fn run_setup(cfg: &mut config::Config) -> Result<(), Box<dyn Error>> {
    let mut api_key = String::new();

    println!("CRAW Setup \n\nCRAW works using LLM API services (current support for GROQ only)");
    println!("Enter your GROQ api key: ");
    io::stdin().read_line(&mut api_key)?;

    //add api to OS keyring
    keyring::add_key("groq_api_key", api_key.trim())?;
    //also add the service type in config file
    cfg.set_api_service(config::Services::GROQ, "craw", "craw-config")?;

    println!("API key set!");

    Ok(())
}

pub async fn prompt_agent(
    cfg: &Config,
    prompt: &str,
    context: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    //todo!! use service type from cfg
    let api_key = keyring::get_key("groq_api_key")?;
    let groq_client = groq::Client::new(api_key)?;
    let groq_agent = groq_client
        .agent("llama-3.3-70b-versatile")
        .preamble(&cfg.system_prompt)
        .build();

    let response = match context {
        Some(context) => {
            groq_agent
                .prompt(format!("context:{} prompt:{}", context, prompt))
                .await?
        }

        None => groq_agent.prompt(prompt).await?,
    };

    Ok(response)
}
