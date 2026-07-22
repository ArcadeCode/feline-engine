use std::time::Duration;

use crate::consts::HISTORIC_SIZE;

pub trait FrequencyScheduler {
    /// Executed the most often return true if an action need to be executed.
    fn update(&mut self) -> bool;
    fn time_until_next(&self) -> Duration;
    #[allow(dead_code)]
    fn get_current_rate(&self) -> f32;
    #[allow(dead_code)]
    fn get_historic(&self) -> &[f32; HISTORIC_SIZE];
}