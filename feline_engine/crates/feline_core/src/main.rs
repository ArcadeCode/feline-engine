use std::time::Duration;
use std::thread;

use tracing::{Level, info};
use tracing_subscriber::{FmtSubscriber};

use  gameloop::frequency_scheduler::FrequencyScheduler;

mod gameloop;
mod consts;

fn main() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Logging system initialized!");


    let signal: bool = true;
    info!("Gameloop started.");

    let mut tick_scheduler = gameloop::tick::TickScheduler::default();
    //let mut frame_scheduler= gameloop::frame::FrameScheduler::new(Some(60.0));
    let mut frame_scheduler = gameloop::frame::FrameScheduler::default();

    while signal  {
        let tick_ready = tick_scheduler.update();
        let frame_ready = frame_scheduler.update();

        if tick_ready {
            // tick logic here
        }
        if frame_ready {
            // frame logic here
        }

        // Waiting for a future action to take place
        let sleep_duration = tick_scheduler.time_until_next().min(frame_scheduler.time_until_next());
        if sleep_duration > Duration::ZERO {
            thread::sleep(sleep_duration);
        }
    }
}