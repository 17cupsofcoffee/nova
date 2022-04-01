use std::time::{Duration, Instant};

pub struct Timer {
    last_time: Instant,
    accumulator: Duration,
    delta_time: Duration,
    fixed_delta: Duration,
    max_lag: Duration,
}

impl Timer {
    pub fn new(tick_rate: f64) -> Timer {
        let fixed_delta = Duration::from_secs_f64(1.0 / tick_rate);

        Timer {
            last_time: Instant::now(),
            accumulator: Duration::ZERO,
            delta_time: Duration::ZERO,
            fixed_delta,
            max_lag: fixed_delta * 8,
        }
    }

    pub fn tick(&mut self) {
        let curr_time = Instant::now();

        self.delta_time = curr_time - self.last_time;
        self.accumulator = Duration::min(self.accumulator + self.delta_time, self.max_lag);
        self.last_time = curr_time;
    }

    pub fn reset(&mut self) {
        self.last_time = Instant::now();
        self.accumulator = Duration::ZERO;
        self.delta_time = Duration::ZERO;
    }

    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    pub fn fixed_delta(&self) -> Duration {
        self.fixed_delta
    }

    pub fn blend_factor(&self) -> f32 {
        self.accumulator.as_secs_f32() / self.fixed_delta.as_secs_f32()
    }

    pub fn check_update_ready(&mut self) -> bool {
        let ready = self.accumulator >= self.fixed_delta;

        if ready {
            self.accumulator -= self.fixed_delta;
        }

        ready
    }
}
