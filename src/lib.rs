use std::{error::Error, io};

use keyring::{Entry, Result as KeyringResult};

pub fn run_setup() -> Result<(), Box<dyn Error>> {
    let mut api_key = String::new();

    println!("CRAW Setup \n\nCRAW works using LLM API services (current support for GROQ only)");
    println!("Enter your GROQ api key: ");

    io::stdin().read_line(&mut api_key)?;
    add_key("groq_api_key", api_key.trim())?;
    println!("API key set!");

    Ok(())
}

fn add_key(service: &str, api_key: &str) -> KeyringResult<()> {
    let entry = Entry::new("craw", service)?;
    entry.set_password(api_key)?;
    Ok(())
}

fn get_key(service: &str) -> KeyringResult<String> {
    let entry = Entry::new("craw", service)?;
    let key = entry.get_password()?;
    Ok(key)
}

fn delete_key(service: &str) -> KeyringResult<()> {
    let entry = Entry::new("craw", service)?;
    entry.delete_credential()?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyring_test() {
        let sample_key = String::from("my-sample-key");
        add_key("sample-service", &sample_key).expect("failed to add key");
        assert_eq!(
            get_key("sample-service").expect("failed to retrieve key"),
            sample_key
        );
        delete_key("sample-service").expect("failed to delete key");
    }
}
