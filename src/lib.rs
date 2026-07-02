pub mod config;
mod keyring;

use std::{error::Error, io};

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
