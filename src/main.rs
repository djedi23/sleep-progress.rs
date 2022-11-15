use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use miette::Result;
use sleep_progress::{parse_interval, Args};
use std::sync::Arc;
use tokio::{
  self,
  time::{sleep, Duration},
};

#[tokio::main]
async fn main() -> Result<()> {
  let args: Args = Args::parse();
  let interval: u64 = parse_interval(&args)?;

  let sleep_task = tokio::spawn(async move {
    sleep(Duration::from_millis(interval)).await;
  });

  if args.progress {
    let pb = Arc::new(ProgressBar::new(interval / 1000));
    pb.set_style(ProgressStyle::with_template("{wide_bar} [{eta_precise}]").unwrap());
    let pbb = pb.clone();

    tokio::spawn(async move {
      for _i in 0..(interval / 1000) {
        sleep(Duration::from_millis(1000)).await;
        pbb.inc(1);
      }
    });
    sleep_task.await.unwrap();
    pb.finish_and_clear();
  } else {
    sleep_task.await.unwrap();
  }
  Ok(())
}
