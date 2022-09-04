use std::ops::{Add, Sub};
use std::time::Duration;

// From wasm-timer crate - https://github.com/tomaka/wasm-timer/blob/master/src/wasm.rs
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Instant {
    /// Unit is milliseconds.
    inner: f64,
}

impl Instant {
    pub fn now() -> Instant {
        return if let Some(window) = web_sys::window() {
            if let Some(performance) = window.performance() {
                let millis = performance.now();
                Instant { inner: millis }
            } else {
                Instant { inner: 0.0 }
            }
        } else {
            Instant { inner: 0.0 }
        };
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, other: Duration) -> Instant {
        let new_val = self.inner + other.as_millis() as f64;
        Instant { inner: new_val as f64 }
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(self, other: Duration) -> Instant {
        let new_val = self.inner - other.as_millis() as f64;
        Instant { inner: new_val as f64 }
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    fn sub(self, other: Instant) -> Duration {
        let ms = self.inner - other.inner;
        if ms >= 0.0 {
            Duration::from_millis(ms as u64)
        } else {
            Duration::from_millis(0)
        }
    }
}