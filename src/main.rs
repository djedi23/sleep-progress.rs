use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use miette::{IntoDiagnostic, Result};
use sleep_progress::{parse_interval, Args};
use std::{sync::Arc, thread, time::Duration};

fn main() -> Result<()> {
  let args: Args = Args::parse();
  let interval: u64 = parse_interval(&args)?;

  if args.progress {
    let pb = Arc::new(ProgressBar::new(interval));
    pb.set_style(ProgressStyle::with_template("{wide_bar} [{eta_precise}]").into_diagnostic()?);
    let pbb = pb.clone();

    thread::spawn(move || -> Result<()> {
      loop {
        thread::sleep(Duration::from_millis(500));
        pbb.set_position(pbb.elapsed().as_millis().try_into().into_diagnostic()?);
      }
    });
    thread::sleep(Duration::from_millis(interval));
    pb.finish_and_clear();
  } else {
    thread::sleep(Duration::from_millis(interval));
  }

  Ok(())
}
