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
    cfg.set_api_service(config::Services::GROQ)?;

    println!("API key set!");

    Ok(())
}

pub async fn prompt_model(prompt: &str, cfg: &Config) -> Result<String, Box<dyn Error>> {
    let groq_client = groq::Client::new("YOUR_API_KEY")?;
    let groq_agent = groq_client.agent("llama-3.3-70b-versatile").build();
    let response = groq_agent.prompt(prompt).await?;
    Ok(response)
}
