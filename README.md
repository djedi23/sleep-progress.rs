# Sleep progress

`sleep-progress` is a clone of GNU sleep with an optional progress bar.

![demo](./sleep-progress.gif)

The arguments are compatible with the original sleep but you can add `--progress` or `-p` to display a progress bar with an ETA.

It can be use as a replacement for GNU sleep: `alias sleep=sleep-progress` .

WARNING: the deplayed ETA may not be as accurate as the sleep delay.

```
Usage: sleep-progress [OPTIONS] <NUMBER>...

Arguments:
  <NUMBER>...  Pause  for  NUMBER seconds. 
               SUFFIX may be 's' for seconds (the default), 'm' for minutes, 'h' for hours or 'd' for days.
               NUMBER need not be an integer.
               Given two or more arguments, pause for the amount of time specified by the sum of their values

Options:
  -p, --progress  Display the sleep indicator
  -h, --help      Print help information
  -V, --version   Print version information
```

## Installation


### From cargo

Ensure Rust is installed: https://www.rust-lang.org/tools/install

``` bash
cargo install sleep-progress
```

### From source

Ensure Rust is installed: https://www.rust-lang.org/tools/install

``` bash
git clone https://github.com/djedi23/sleep-progress.rs.git
cd sleep-progress.rs
cargo install --path .
```
