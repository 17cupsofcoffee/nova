use std::time::{Duration, Instant};

pub struct Timer {
    last_time: Instant,
    accumulator: Duration,
    fixed_delta: Duration,
    max_lag: Duration,
}

impl Timer {
    pub fn new(tick_rate: f64) -> Timer {
        let fixed_delta = Duration::from_secs_f64(1.0 / tick_rate);

        Timer {
            last_time: Instant::now(),
            accumulator: Duration::ZERO,
            fixed_delta,
            max_lag: fixed_delta * 8,
        }
    }

    pub fn tick(&mut self) {
        let curr_time = Instant::now();
        let delta_time = curr_time - self.last_time;

        self.accumulator = Duration::min(self.accumulator + delta_time, self.max_lag);
        self.last_time = curr_time;
    }

    pub fn delta(&self) -> Duration {
        self.fixed_delta
    }

    pub fn check_update_ready(&mut self) -> bool {
        let ready = self.accumulator >= self.fixed_delta;

        if ready {
            self.accumulator -= self.fixed_delta;
        }

        ready
    }
}
