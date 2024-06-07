Implements automatic function profiling via the `#[profile]` macro.

# Use

Annotate a function with `#[profile]`.
The execution time will be automatically logged as the fractional number of seconds reported by `Instant::elapsed` from the start of the function.

The macro is only expanded if `debug_assertions` is true.
By default, this is not true in release builds, but can be enabled by adding

```ignore
[profile.release]
debug-assertions=true
```

to Cargo.toml.

The profiling object is only created if its log level is enabled for the current active level.

```rust
use proflogger::*;

#[profile]
fn func1() {
    std::thread::sleep(std::time::Duration::from_secs(1));

    // will log
    // func1: 1.000000000
    // at log::Level::Trace
}

// By default, the log level is set to `log::level::Trace`,
// but this can be customized like

#[profile("Error")]
fn expensive_function(arg1: usize, arg2: usize) -> usize {
    (arg1..arg2).map(|a| a * a).sum()
}
```

