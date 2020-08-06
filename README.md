# jumpto

![CI](https://github.com/Celeo/jumpto/workflows/CI/badge.svg?branch=master)
![Crates.io](https://img.shields.io/crates/v/jumpto.svg)
![License](https://img.shields.io/crates/l/jumpto)

Simple CLI utility to navigate to directories.

## Installing

```sh
cargo install jumpto
```

## Using

Since shells spawn subprocesses for running commands, there's an extra step required to set up this program. Running

```sh
jumpto script
```

shows a Bash script that takes the runtime arguments, runs the program, looks to see if it needs to change directory, and then does so. You can name this script 'jt' with something like:

```sh
jumpto script > ~/.cargo/bin/jt
chmod +x ~/.cargo/bin/jt
```

Then, calling the program can be done via:

```sh
source jt <arguments>
```

The next improvement will remove the necessity to include 'source ' when calling the script.

## Developing

### Building

### Requirements

* Git
* A recent version of [Rust](https://www.rust-lang.org/tools/install)

### Steps

```sh
git clone https://github.com/Celeo/jumpto
cd jumpto
cargo build
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Please feel free to contribute. Please open an issue first (or comment on an existing one) so that I know that you want to add/change something.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
