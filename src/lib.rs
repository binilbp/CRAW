mod keyring;

use std::{error::Error, io};

pub fn run_setup() -> Result<(), Box<dyn Error>> {
    let mut api_key = String::new();

    println!("CRAW Setup \n\nCRAW works using LLM API services (current support for GROQ only)");
    println!("Enter your GROQ api key: ");

    io::stdin().read_line(&mut api_key)?;
    keyring::add_key("groq_api_key", api_key.trim())?;
    println!("API key set!");

    Ok(())
}

pub fn is_setup_done() -> Result<(), &'static str> {
    //TODO: add real logic
    let done_setup = true;
    if !done_setup {
        return Err("CRAW setup not done. Please complete setup first. Run `craw setup`");
    }
    Ok(())
}
