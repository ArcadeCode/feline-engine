/// Represent the maximum frequency values to store for generating a frequency average value per seconds.
pub const HISTORIC_SIZE: usize = 10;

/// Fixed Tick per seconds, cannot be changed post-compilation.
/// By default set to 60 TPS like the common engines of the sector.
pub const FIXED_TPS: f32 = 60.0; // target TPS