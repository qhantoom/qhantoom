// use super::interface::{LOG_INCOGNITO, LOG_NAME};

#[inline]
pub fn username() -> String {
  // tmp for demo time
  format!("{}", "incognito")
  // std::env::var(LOG_NAME).unwrap_or(String::from(LOG_INCOGNITO))
}
