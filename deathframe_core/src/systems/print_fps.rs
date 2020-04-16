use super::system_prelude::*;
use amethyst::utils::fps_counter::FpsCounter;
use std::time::{Duration, Instant};

/// Prints the current and average FPS to `stderr`.
pub struct PrintFpsSystem {
    print_delay: Duration,
    last_print:  Instant,
}

impl<'a> System<'a> for PrintFpsSystem {
    type SystemData = Read<'a, FpsCounter>;

    fn run(&mut self, fps_counter: Self::SystemData) {
        let now = Instant::now();

        if now - self.last_print >= self.print_delay {
            let fps_frame = fps_counter.frame_fps();
            let fps_avg = fps_counter.sampled_fps();
            eprintln!("[FPS] {:.02} avg: {:.02})", fps_frame, fps_avg);
            self.last_print = now;
        }
    }
}

impl PrintFpsSystem {
    /// Set the _print delay_ with this builder function.
    pub fn with_print_delay(mut self, print_delay: Duration) -> Self {
        self.print_delay = print_delay;
        self
    }
}

impl Default for PrintFpsSystem {
    fn default() -> Self {
        Self {
            print_delay: Duration::from_secs(1),
            last_print:  Instant::now(),
        }
    }
}
