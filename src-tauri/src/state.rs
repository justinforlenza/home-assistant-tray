use serde::{Deserialize, Serialize};

use tauri::Url;

use toml;

#[derive(Deserialize)]
pub struct ConfigBase {
    pub url: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub url: Option<String>,
    #[serde(skip_serializing)]
    path: String
}


impl Config {
  pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
    println!("===============Config Loading=================");
    println!("Loading config from: {}", path);
    let content = Self::load_from_file(path)?;
    println!("Config content: {:?}", content);
    let mut config: ConfigBase = toml::from_str(&content)?;
    if config.url.is_some() {
      println!("Validating URL: {}", config.url.clone().unwrap());
      let url = Url::parse(&config.url.clone().unwrap());

      if url.is_err() {
        println!("Invalid URL, resetting to None");
        config.url = None;
      }
    }
    let config = Config {
      url: config.url,
      path: path.to_string(),
    };
    println!("Final Config: {:?}", config);
    println!("============================================");
    Ok(config)
  }

  fn load_from_file(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    match std::fs::read_to_string(path) {
      Ok(content) => Ok(content),
      Err(_) => {
        let default_content = "";
        std::fs::write(path, "")?;
        Ok(default_content.to_string())
      }
    }
  }

  // fn reload_from_file(&self) -> Result<String, Box<dyn std::error::Error>> {
  //   Self::load_from_file(&self.path)
  // }

  fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
    let toml_string = toml::to_string(self)?;
    std::fs::write(&self.path, toml_string)?;
    Ok(())
  }

  pub fn update_url(&mut self, new_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("=============URL Config Update==============");
    println!("Updating URL to: {}", new_url);
    let parsed_url = Url::parse(new_url);
    if parsed_url.is_err() {
      println!("Invalid URL format: {}", new_url);
      println!("============================================");
      return Err("Invalid URL format".into());
    }
    self.url = Some(new_url.to_string());
    println!("Updated Config: {:?}", self);
    self.save_to_file()?;
    println!("Config saved to file: {}", self.path);
    println!("============================================");
    Ok(())
  }
}
