use std::time::{Duration, Instant};
use tracing::trace;

use crate::consts::{FIXED_TPS, HISTORIC_SIZE};
use crate::gameloop::frequency_scheduler::FrequencyScheduler;

#[derive(Debug)]
pub struct TickScheduler {
    /// Accumulated time since the last tick (in seconds)
    accumulator: f32,
    /// Time between each tick (1.0 / FIXED_TPS)
    tick_interval: f32,
    /// Nombre de ticks exécutés cette seconde
    ticks_this_second: u32,
    /// TPS historic (in ticks/second)
    historic: [f32; HISTORIC_SIZE],
    /// Instant of the last update
    last_update: Instant,
    /// Instant from the start of the current second
    last_second: Instant,
}

impl Default for TickScheduler {
    fn default() -> Self {
        Self {
            accumulator: 0.0,
            tick_interval: 1.0 / FIXED_TPS,
            ticks_this_second: 0,
            historic: [0.0; HISTORIC_SIZE],
            last_update: Instant::now(),
            last_second: Instant::now(),
        }
    }
}

impl FrequencyScheduler for TickScheduler {
    /// Method called each frame to update the TickManger.
    /// Return true if a tick need to be executed.
    fn update(&mut self) -> bool {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Elapsed time accumulator.
        self.accumulator += delta_time;

        // Check if one second elapsed to update the historic.
        if now.duration_since(self.last_second) >= Duration::from_secs(1) {
            let tps = self.ticks_this_second as f32;

            // Offset the historic
            for i in 1..HISTORIC_SIZE {
                self.historic[i - 1] = self.historic[i];
            }
            self.historic[HISTORIC_SIZE - 1] = tps;

            self.ticks_this_second = 0;
            self.last_second = now;

            trace!(tps, accumulator = ?self.accumulator, "TPS updated");
        }

        // Check for tick to be executed
        if self.accumulator >= self.tick_interval {
            self.accumulator -= self.tick_interval;
            self.ticks_this_second += 1;
            true
        } else {
            false
        }
    }

    fn time_until_next(&self) -> Duration {
        let remaining = self.tick_interval - self.accumulator;
        if remaining <= 0.0 {
            Duration::ZERO
        } else {
            Duration::from_secs_f32(remaining)
        }
    }

    #[allow(dead_code)]
    fn get_historic(&self) -> &[f32; HISTORIC_SIZE] {
        &self.historic
    }

    #[allow(dead_code)]
    fn get_current_rate(&self) -> f32 {
        self.ticks_this_second as f32
    }
}