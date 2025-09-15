use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Environment {
    Test,
    Live,
}

impl Environment {
    pub(crate) fn base_url(&self) -> &str {
        match self {
            Environment::Test => "https://test.dodopayments.com",
            Environment::Live => "https://live.dodopayments.com",
        }
    }
}

impl From<&str> for Environment {
    fn from(mode: &str) -> Self {
        match mode {
            "test_mode" => Environment::Test,
            "live_live" => Environment::Live,
            _ => panic!("Invalid mode: {}", mode),
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Test => write!(f, "test_mode"),
            Environment::Live => write!(f, "live_live"),
        }
    }
}
