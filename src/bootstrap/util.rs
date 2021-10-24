use std::fs;
use std::io;
use std::path::Path;

use chrono::{DateTime, FixedOffset, Utc};

// get the current date time
#[inline]
pub fn datetime() -> String {
  let now = Utc::now();
  let tz = FixedOffset::east(2 * 3600); // TODO: detect current timezone
  let utc_time = DateTime::<Utc>::from_utc(now.naive_utc(), Utc);

  utc_time
    .with_timezone(&tz)
    .format("%B %d %Y, %H:%M:%S")
    .to_string()
}

// read the file at the given path and return the contents as a string
#[inline]
pub fn readfile(pathname: &str) -> Result<String, String> {
  let path = Path::new(pathname);

  match fs::read_to_string(path) {
    Ok(file) => Ok(file),
    Err(e) => Err(format!("{}", e)),
  }
}

// read the lines from stdin at the given path and return the contents as a string
#[inline]
pub fn readline<'a>(icon: &str) -> Result<String, String> {
  let stdin = io::stdin();
  let mut stdout = io::stdout();
  let mut buf = String::new();

  print!("{} ", icon);

  io::Write::flush(&mut stdout).expect("flush failed!");
  buf.clear();
  stdin.read_line(&mut buf).unwrap();
  buf.truncate(buf.trim_end().len());

  Ok(buf)
}

// get the user name from the system
#[inline]
pub fn username() -> String {
  const LOG_NAME: &str = "LOGNAME";
  const LOG_INCOGNITO: &str = "johndoe";

  std::env::var(LOG_NAME).unwrap_or(String::from(LOG_INCOGNITO))
}
