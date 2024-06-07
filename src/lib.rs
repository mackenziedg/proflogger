#![doc = include_str!("../README.md")]
mod profiler;

pub use profiler::AutoLogger;
pub use proflogger_proc::profile;
