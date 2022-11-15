#![no_main]
use libfuzzer_sys::fuzz_target;
use sleep_progress::{parse_interval, Args};

fuzz_target!(|data: Args| {
  parse_interval(&data);
});
