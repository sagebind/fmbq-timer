use std::time::{Duration, Instant};

pub struct Timer {
    active: bool,
    now: Instant,
    end_time: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            active: false,
            now: Instant::now(),
            end_time: Instant::now(),
        }
    }

    pub fn update(&mut self) -> UpdateResult {
        if self.active {
            let remaining = self.end_time.saturating_duration_since(Instant::now());

            if remaining.is_zero() {
                self.active = false;
                UpdateResult::Expired
            } else {
                UpdateResult::Running(remaining)
            }
        } else {
            UpdateResult::Inactive
        }
    }

    pub fn start(&mut self, duration: Duration) {
        self.now = Instant::now();
        self.end_time = self.now + duration;
        self.active = true;
    }

    pub fn reset(&mut self) {
        self.active = false;
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpdateResult {
    Running(Duration),
    Expired,
    Inactive,
}
