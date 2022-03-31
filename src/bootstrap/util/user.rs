// use super::interface::{LOG_INCOGNITO, LOG_NAME};

pub fn username() -> String {
  // tmp for demo time
  format!("{}", "incognito")
  // std::env::var(LOG_NAME).unwrap_or(String::from(LOG_INCOGNITO))
}
