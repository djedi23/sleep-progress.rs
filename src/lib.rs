use clap::Parser;
use miette::{Diagnostic, Result};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("invalid time interval '{origin}'")]
#[diagnostic(
  code(invalid::time),
  help("Try `sleep-progress --help` for more informations.")
)]
pub struct InvalidTimeInterval {
  origin: String,
}

#[derive(Parser, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[command(author, version, about, long_about = None)]
pub struct Args {
  /// Pause  for  NUMBER seconds.  SUFFIX may be 's' for seconds (the default), 'm' for minutes, 'h' for hours or 'd' for days.  NUMBER need not be an integer.  Given two or more arguments, pause for the amount of time specified by the sum of their values.
  #[arg(required = true)]
  number: Vec<String>,

  /// Display the sleep indicator
  #[arg(short, long)]
  pub progress: bool,
}

pub fn parse_interval(args: &Args) -> Result<u64> {
  let mut sum = 0.0;
  for duration_spec in args.number.iter() {
    let (value, multipliers) = if let Some(seconds) = duration_spec.strip_suffix('s') {
      (seconds, 1000.0)
    } else if let Some(minutes) = duration_spec.strip_suffix('m') {
      (minutes, 60.0 * 1000.0)
    } else if let Some(hours) = duration_spec.strip_suffix('h') {
      (hours, 60.0 * 60.0 * 1000.0)
    } else if let Some(days) = duration_spec.strip_suffix('d') {
      (days, 24.0 * 60.0 * 60.0 * 1000.0)
    } else {
      (duration_spec.as_str(), 1000.0)
    };
    sum += multipliers
      * value.parse::<f64>().map_err(|_| InvalidTimeInterval {
        origin: duration_spec.to_string(),
      })?
  }
  Ok(sum.round() as u64)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_help() {
    let result = Args::try_parse_from([" ", "-h"]);
    assert!(result.is_err());
  }

  #[test]
  fn parse_long_help() {
    let result = Args::try_parse_from([" ", "--help"]);
    assert!(result.is_err());
  }

  #[test]
  fn parse_unknown_args() {
    let result = Args::try_parse_from([" ", "-t"]);
    //    dbg!(&result);
    assert!(result.is_err());
  }

  #[test]
  fn parse_cli_arg() {
    let result = Args::try_parse_from([" ", "34"]);
    //    dbg!(&result);
    assert!(result.is_ok());
  }

  #[test]
  fn parse_cli_args() {
    let result = Args::try_parse_from([" ", "34", "6.4"]);
    //    dbg!(&result);
    assert!(result.is_ok());
  }
  #[test]
  fn parse_cli_arg_progress() {
    let result = Args::try_parse_from([" ", "34", "-p"]);
    //    dbg!(&result);
    assert!(result.is_ok());
  }

  #[test]
  fn parse_cli_args_progress() {
    let result = Args::try_parse_from([" ", "34", "6.4", "-p"]);
    //    dbg!(&result);
    assert!(result.is_ok());
  }

  #[test]
  fn parse_cli_arg_unknown_args() {
    let result = Args::try_parse_from([" ", "34", " ", "-t"]);
    //dbg!(&result);
    assert!(result.is_err());
  }

  #[test]
  fn parse_cli_args_unknown_args() {
    let result = Args::try_parse_from([" ", "34", "6.4", " ", "-t"]);
    //    dbg!(&result);
    assert!(result.is_err());
  }
  #[test]
  fn parse_cli_arg_progress_unknown_args() {
    let result = Args::try_parse_from([" ", "34", "-p", " ", "-t"]);
    //    dbg!(&result);
    assert!(result.is_err());
  }

  #[test]
  fn parse_cli_args_progress_unknown_args() {
    let result = Args::try_parse_from([" ", "34", "6.4", "-p", " ", "-t"]);
    //    dbg!(&result);
    assert!(result.is_err());
  }

  #[test]
  fn parse_interval_1() {
    let result = parse_interval(&Args {
      number: vec!["1".into()],
      progress: false,
    });
    assert_eq!(result.ok(), Some(1000));
  }

  #[test]
  fn parse_interval_1p() {
    let result = parse_interval(&Args {
      number: vec!["1".into()],
      progress: true,
    });
    assert_eq!(result.ok(), Some(1000));
  }

  #[test]
  fn parse_interval_0_5() {
    let result = parse_interval(&Args {
      number: vec!["0.5".into()],
      progress: false,
    });
    assert_eq!(result.ok(), Some(500));
  }

  #[test]
  fn parse_interval_1s() {
    let result = parse_interval(&Args {
      number: vec!["1s".into()],
      progress: false,
    });
    assert_eq!(result.ok(), Some(1000));
  }

  #[test]
  fn parse_interval_1m() {
    let result = parse_interval(&Args {
      number: vec!["1m".into()],
      progress: false,
    });
    assert_eq!(result.ok(), Some(60000));
  }

  #[test]
  fn parse_interval_1h() {
    let result = parse_interval(&Args {
      number: vec!["1h".into()],
      progress: false,
    });
    assert_eq!(result.ok(), Some(3600000));
  }

  #[test]
  fn parse_interval_1d() {
    let result = parse_interval(&Args {
      number: vec!["1d".into()],
      progress: false,
    });
    assert_eq!(result.ok(), Some(86400000));
  }

  #[test]
  fn parse_interval_multiple() {
    let result = parse_interval(&Args {
      number: vec![
        "1.023".into(),
        "1s".into(),
        "1m".into(),
        "1h".into(),
        "1d".into(),
      ],
      progress: false,
    });
    assert_eq!(result.ok(), Some(90062023));
  }

  #[test]
  fn parse_interval_err() {
    let result = parse_interval(&Args {
      number: vec!["1z".into()],
      progress: false,
    });

    assert_eq!(
      result.err().unwrap().to_string(),
      "invalid time interval '1z'"
    );
  }

  #[test]
  fn parse_interval_err_2() {
    let result = parse_interval(&Args {
      number: vec![
        "1".into(),
        "2".into(),
        "3e".into(),
        "4".into(),
        "5".into(),
        "6".into(),
      ],
      progress: false,
    });

    assert_eq!(
      result.err().unwrap().to_string(),
      "invalid time interval '3e'"
    );
  }

  #[test]
  fn parse_interval_err_3() {
    let result = parse_interval(&Args {
      number: vec!["one".into()],
      progress: false,
    });

    assert_eq!(
      result.err().unwrap().to_string(),
      "invalid time interval 'one'"
    );
  }
}
