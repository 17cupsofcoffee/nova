use std::time::{Duration, Instant};

pub struct Timer {
    last_time: Instant,
    accumulated_time: Duration,
    target_time: Duration,
    max_lag: Duration,
}

impl Timer {
    pub fn new(tick_rate: f64) -> Timer {
        let target_time = Duration::from_secs_f64(1.0 / tick_rate);

        Timer {
            last_time: Instant::now(),
            accumulated_time: Duration::ZERO,
            target_time,
            max_lag: target_time * 8,
        }
    }

    pub fn tick(&mut self) {
        self.advance_time();
        self.cap_accumulated_time();
    }

    pub fn tick_until_update_ready(&mut self) {
        self.advance_time();

        // TODO: This isn't accurate enough - need to sleep and then spin.
        while self.accumulated_time < self.target_time {
            std::thread::sleep(Duration::from_millis(1));

            self.advance_time();
        }

        self.cap_accumulated_time();
    }

    pub fn reset(&mut self) {
        self.last_time = Instant::now();
        self.accumulated_time = Duration::ZERO;
    }

    pub fn check_update_ready(&mut self) -> bool {
        let ready = self.accumulated_time >= self.target_time;

        if ready {
            self.accumulated_time -= self.target_time;
        }

        ready
    }

    pub fn delta(&self) -> Duration {
        self.target_time
    }

    pub fn blend_factor(&self) -> f32 {
        self.accumulated_time.as_secs_f32() / self.target_time.as_secs_f32()
    }

    fn advance_time(&mut self) {
        let current_time = Instant::now();
        let time_advanced = current_time - self.last_time;

        self.accumulated_time += time_advanced;
        self.last_time = current_time;
    }

    fn cap_accumulated_time(&mut self) {
        if self.accumulated_time > self.max_lag {
            self.accumulated_time = self.max_lag;
        }
    }
}
