#![no_main]
use clap::Parser;
use libfuzzer_sys::fuzz_target;
use sleep_progress::Args;

fuzz_target!(|data: Vec<String>| {
  Args::try_parse_from(data);
});
