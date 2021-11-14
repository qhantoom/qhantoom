use super::interface::{LOG_INCOGNITO, LOG_NAME};

#[inline]
pub fn username() -> String {
  std::env::var(LOG_NAME).unwrap_or(String::from(LOG_INCOGNITO))
}
