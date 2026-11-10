//! Memory Footprint, Allocation Tracker, and Execution Profiler for Kora.

use std::time::Instant;

pub struct Profiler;

impl Profiler {
    pub fn measure_ms<F: FnOnce() -> R, R>(label: &str, f: F) -> (R, u128) {
        let start = Instant::now();
        let result = f();
        let elapsed = start.elapsed().as_millis();
        println!("[Profiler] {label}: {elapsed} ms");
        (result, elapsed)
    }
}
