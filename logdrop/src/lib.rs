use std::time::Instant;

pub use logdrop_proc::profile;

pub struct LogDropProfiler {
    name: &'static str,
    start: Instant,
    level: log::Level,
}

impl LogDropProfiler {
    #[must_use]
    pub fn new(name: &'static str, level: log::Level) -> Self {
        Self {
            name,
            start: Instant::now(),
            level,
        }
    }
}

impl Drop for LogDropProfiler {
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

    #[test]
    fn test_macro() {
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
