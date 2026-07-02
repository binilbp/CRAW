use std::{error::Error, io};

use keyring::{Entry, Result as KeyringResult};

pub fn run_setup() -> Result<(), Box<dyn Error>> {
    let mut api_key = String::new();

    println!("CRAW Setup \n\nCRAW works using LLM API services (current support for GROQ only)");
    println!("Enter your GROQ api key: ");

    io::stdin().read_line(&mut api_key)?;
    add_key(api_key.trim())?;
    println!("API key set!");

    Ok(())
}

fn add_key(api_key: &str) -> KeyringResult<()> {
    let entry = Entry::new("craw", "groq_api_key")?;
    entry.set_password(api_key)?;
    // entry.delete_credential()?;
    Ok(())
}

fn get_key() -> KeyringResult<String> {
    let entry = Entry::new("craw", "groq_api_key")?;
    let key = entry.get_password()?;
    Ok(key)
}

pub fn is_setup_done() -> Result<(), &'static str> {
    //TODO: add real logic
    let done_setup = true;
    if !done_setup {
        return Err("CRAW setup not done. Please complete setup first. Run `craw setup`");
    }
    Ok(())
}
