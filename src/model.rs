use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum ActionType {
    XML,
    JSON,
    Text,
    Redirect,
}

impl Default for ActionType {
    fn default() -> Self {
        Self::Text
    }
}

impl ActionType {
    pub fn content_type(&self) -> String {
        return match self {
            Self::XML => "application/xml".to_string(),
            Self::JSON => "application/json".to_string(),
            Self::Text => "text/plain".to_string(),
            Self::Redirect => "text/plain".to_string(),
        };
    }

    pub fn build_data(&self, mut data: String) -> String {
        return match self {
            Self::XML => {
                data.insert_str(0, "<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
                data
            }
            Self::JSON => {
                data.insert(0, '{');
                data.push('}');
                data
            }
            _ => data,
        };
    }
}

// Config struct

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_status_code")]
    pub status_code: u16,
    #[serde(default = "default_log")]
    pub log: bool,
    #[serde(default)]
    pub action_type: ActionType,
    pub action_data: String,
}

fn default_port() -> u16 {
    80
}

fn default_status_code() -> u16 {
    404
}

fn default_log() -> bool {
    true
}
