use keyring::{Entry, Result as KeyringResult};

pub fn add_key(service_name: &str, api_key: &str) -> KeyringResult<()> {
    let entry = Entry::new("craw", service_name)?;
    entry.set_password(api_key)?;
    Ok(())
}

pub fn get_key(service_name: &str) -> KeyringResult<String> {
    let entry = Entry::new("craw", service_name)?;
    let key = entry.get_password()?;
    Ok(key)
}

pub fn delete_key(service_name: &str) -> KeyringResult<()> {
    let entry = Entry::new("craw", service_name)?;
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
