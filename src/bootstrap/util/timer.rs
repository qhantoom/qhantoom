pub fn date_time() -> String {
  use chrono::{DateTime, FixedOffset, Utc};

  let now = Utc::now();
  let tz = FixedOffset::east(2 * 3600); // TODO: detect current timezone
  let utc_time = DateTime::<Utc>::from_utc(now.naive_utc(), Utc);

  utc_time
    .with_timezone(&tz)
    .format("%B %d %Y, %H:%M:%S")
    .to_string()
}
