use chrono::{DateTime, Duration, FixedOffset, Local, TimeDelta, Utc};
use spin::mutex::SpinMutex;

pub fn now() -> DateTime<Local> {
    let dt = Utc::now().naive_utc();
    let offset = FixedOffset::east_opt(8 * 3600).unwrap();
    DateTime::<Local>::from_naive_utc_and_offset(dt, offset)
}
