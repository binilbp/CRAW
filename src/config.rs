use std::error::Error;

use confy::ConfyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api_service: Option<Services>,
    pub system_prompt: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Services {
    GROQ,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            api_service: None,
            system_prompt:
                "your name is `CRAW `. you are a text-based-llm accessible from user terminal"
                    .into(),
        }
    }
}

impl Config {
    pub fn set_api_service(
        &mut self,
        service: Services,
        app_name: &str,
        file_name: &str,
    ) -> Result<(), ConfyError> {
        self.api_service = Some(service);
        confy::store(app_name, file_name, self)?;
        Ok(())
    }

    pub fn set_sys_prompt(
        &mut self,
        sys_prompt: &str,
        app_name: &str,
        file_name: &str,
    ) -> Result<(), ConfyError> {
        self.system_prompt = sys_prompt.into();
        confy::store(app_name, file_name, self)?;
        Ok(())
    }

    pub fn reset_config(&self, app_name: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
        //delete the config file, api and let config file build from default in next start as fresh.
        let config_file_path = confy::get_configuration_file_path(app_name, file_name)?;
        std::fs::remove_file(config_file_path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use confy::get_configuration_file_path;

    use super::*;

    #[test]
    fn config_test() {
        let cfg_name = "craw-test-config";
        let app_name = "craw";
        let mut cfg: Config = confy::load(app_name, cfg_name).unwrap();
        assert!(&cfg.api_service.is_none());
        assert_eq!(
            &cfg.system_prompt,
            "your name is `CRAW `. you are a text-based-llm accessible from user terminal"
        );

        cfg.set_api_service(Services::GROQ, app_name, cfg_name)
            .unwrap();
        cfg.set_sys_prompt("sample_sys_prompt", app_name, cfg_name)
            .unwrap();

        let cfg: Config = confy::load(app_name, cfg_name).unwrap();
        assert_eq!(cfg.api_service, Some(Services::GROQ));
        assert_eq!(cfg.system_prompt, "sample_sys_prompt".to_string());

        let file_path = get_configuration_file_path(app_name, cfg_name).unwrap();
        std::fs::remove_file(file_path).unwrap();
    }
}
