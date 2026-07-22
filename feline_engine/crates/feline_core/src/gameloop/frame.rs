use std::time::{Duration, Instant};
use tracing::trace;

use crate::{consts::{HISTORIC_SIZE}, gameloop::frequency_scheduler::FrequencyScheduler};

#[derive(Debug)]
pub struct FrameScheduler {
    /// Accumulated time since the last frame (in seconds)
    accumulator: f32,
    /// Optional fixed framerate
    fixed_fps: Option<f32>,
    /// Time between each frame (1.0 / FIXED_TPS)
    frame_interval: f32,
    /// Nombre de frames exécutés cette seconde
    frames_this_second: u32,
    /// TPS historic (in frames/second)
    historic: [f32; HISTORIC_SIZE],
    /// Instant of the last update
    last_update: Instant,
    /// Instant from the start of the current second
    last_second: Instant,
}

impl Default for FrameScheduler {
    fn default() -> Self {
        Self {
            accumulator: 0.0,
            fixed_fps: Option::None, // Setting no limit to the FPS
            frame_interval: 1.0 / f32::MAX, // Basically setting a fixed FPS target to an impossible goal.
            frames_this_second: 0,
            historic: [0.0; HISTORIC_SIZE],
            last_update: Instant::now(),
            last_second: Instant::now(),
        }
    }
}

impl FrameScheduler {
    #[allow(dead_code)]
    pub fn new(fixed_fps: Option<f32>) -> Self {
        Self {
            accumulator: 0.0,
            fixed_fps: fixed_fps,
            frame_interval: match fixed_fps {
                Some(fps) => 1.0 / fps, // If fixed, set the fps target.
                None => 1.0 / f32::MAX, // If not, set the fps target to the theoretical maximum.
            },
            frames_this_second: 0,
            historic: [0.0; HISTORIC_SIZE],
            last_update: Instant::now(),
            last_second: Instant::now(),
        }
    }
}

impl FrequencyScheduler for FrameScheduler {
    /// Method called each frame to update the TickManger.
    /// Return true if a frame need to be executed.
    fn update(&mut self) -> bool {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Elapsed time accumulator.
        // Disabled when having f32::MAX fps target the possibility to have an infinite positive accumulation.
        self.accumulator += delta_time;
        if self.fixed_fps.is_none() {
            self.accumulator = 1.0;
        }

        // Check if one second elapsed to update the historic.
        if now.duration_since(self.last_second) >= Duration::from_secs(1) {
            let fps = self.frames_this_second as f32;

            // Offset the historic
            for i in 1..HISTORIC_SIZE {
                self.historic[i - 1] = self.historic[i];
            }
            self.historic[HISTORIC_SIZE - 1] = fps;

            self.frames_this_second = 0;
            self.last_second = now;

            if let Some(target_fps) = self.fixed_fps {
                trace!("{}/{} FPS updated | accumulator={:.6}", fps, target_fps, self.accumulator);
            } else {
                trace!("{}/+inf FPS updated | accumulator={:.6}", fps, self.accumulator);
            }   
        }

        // Check for frame to be executed
        if self.accumulator >= self.frame_interval {
            self.accumulator -= self.frame_interval;
            self.frames_this_second += 1;
            true
        } else {
            false
        }
    }

    fn time_until_next(&self) -> Duration {
        match self.fixed_fps {
            None => Duration::ZERO,  // Uncapped when no FPS limit established
            Some(_) => {
                let remaining = self.frame_interval - self.accumulator;
                if remaining <= 0.0 { Duration::ZERO } else { Duration::from_secs_f32(remaining) }
            }
        }
    }

    #[allow(dead_code)]
    fn get_historic(&self) -> &[f32; HISTORIC_SIZE] {
        &self.historic
    }

    #[allow(dead_code)]
    fn get_current_rate(&self) -> f32 {
        self.frames_this_second as f32
    }
}