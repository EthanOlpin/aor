use std::time::{Duration, Instant};

pub struct BenchResult<T> {
    pub result: T,
    pub duration: Duration,
}

pub fn it<T>(f: impl FnOnce() -> T) -> BenchResult<T> {
    let start = Instant::now();
    let result = std::hint::black_box(f());
    let duration = start.elapsed();
    BenchResult { result, duration }
}
