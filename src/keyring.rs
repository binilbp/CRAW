use keyring::{Entry, Result as KeyringResult};

pub fn add_key(service: &str, api_key: &str) -> KeyringResult<()> {
    let entry = Entry::new("craw", service)?;
    entry.set_password(api_key)?;
    Ok(())
}

pub fn get_key(service: &str) -> KeyringResult<String> {
    let entry = Entry::new("craw", service)?;
    let key = entry.get_password()?;
    Ok(key)
}

pub fn delete_key(service: &str) -> KeyringResult<()> {
    let entry = Entry::new("craw", service)?;
    entry.delete_credential()?;
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
