use std::time::Instant;

/// Starts a timer when created which logs the elapsed lifetime when it is dropped.
///
/// While this can be created and used manually, the [`proflogger_proc::profile`] macro from proflogger-proc is the more ergonomic method of use.
pub struct AutoLogger {
    name: &'static str,
    start: Instant,
    level: log::Level,
}

impl AutoLogger {
    #[must_use]
    pub fn new(name: &'static str, level: log::Level) -> Self {
        Self {
            name,
            start: Instant::now(),
            level,
        }
    }
}

impl Drop for AutoLogger {
    fn drop(&mut self) {
        log::log!(
            self.level,
            "{}: {}",
            self.name,
            self.start.elapsed().as_secs_f64()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile;

    #[test]
    fn test_macro() {
        // These tests have to be tested manually for now.
        // First, setting RUST_LOG=trace should show all functions print profiles.
        // Setting RUST_LOG=warn should show only function2.
        // Running cargo test --release should not show any.

        env_logger::init();

        #[profile]
        fn test_profiled_function() {
            let a = 0;
            println!("{a}");
        }

        #[profile("Error")]
        fn test_profiled_function2() {
            let a = 0;
            println!("{a}");
        }

        #[profile]
        fn test_profiled_function3(a: i32) -> i32 {
            println!("{a}");
            a
        }

        #[profile]
        pub fn test_profiled_function4<T>(a: T) -> T {
            println!("0");
            return a;
        }

        #[profile]
        fn test_profiled_function5(a: usize) -> usize {
            (0..a).sum()
        }

        test_profiled_function();
        test_profiled_function2();
        test_profiled_function3(0);
        test_profiled_function4(0);
        test_profiled_function5(1_000_000);
    }
}
