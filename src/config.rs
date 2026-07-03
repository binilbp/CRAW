use confy::ConfyError;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_service: Option<Services>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Services {
    GROQ,
}

impl Config {
    pub fn set_api_service(&mut self, service: Services) -> Result<(), ConfyError> {
        self.api_service = Some(service);
        confy::store("craw", "craw-config", self)?;
        Ok(())
    }
}
