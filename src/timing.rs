use std::time::{Duration, SystemTime, SystemTimeError};

pub struct BenchResult<T> {
    pub result: T,
    pub duration: Duration,
}

pub fn it<T>(f: impl FnOnce() -> T) -> Result<BenchResult<T>, SystemTimeError> {
    let start = SystemTime::now();
    let result = f();
    let duration = start.elapsed()?;
    Ok(BenchResult { result, duration })
}
