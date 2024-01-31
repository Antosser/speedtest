# Speedtest

Speedtest is a Rust-based command-line utility designed for measuring network speed between two computers. It provides two subcommands: `serve` and `test`, allowing users to set up a server on one machine and perform network speed tests from another machine.

## Installation

To install Speedtest, use the following command:

```bash
cargo install speedtest
```

## Usage

```
Rust-based network speed testing tool between two computers

Usage: speedtest <COMMAND>

Commands:
  serve  Listen to incoming TCP connections on the given socket address
  test   Connect to a TCP server with the given socket address
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Serve

```
Listen to incoming TCP connections on the given socket address

Usage: speedtest serve <SOCKET>

Arguments:
  <SOCKET>  Socket address

Options:
  -h, --help  Print help
```

### Test

```
Connect to a TCP server with the given socket address

Usage: speedtest test [OPTIONS] <SOCKET>

Arguments:
  <SOCKET>  Socket address

Options:
  -l, --length <LENGTH>  How much bytes to send to the server in MEGABYTES [default: 10]
  -h, --help             Print help
```

## Examples

1. Set up a server on one computer:

```bash
speedtest serve 127.0.0.1:8080
```

2. Run a speed test from another computer (by sending 100 MB (Megabytes)):

```bash
speedtest test 127.0.0.1:8080 --length 100
```
