# Speedtest

Speedtest is a Rust-based command-line utility designed for measuring network speed between two computers. It provides two subcommands: `serve` and `test`, allowing users to set up a server on one machine and perform network speed tests from another machine.

## Installation

To install Speedtest, use the following command:

```bash
cargo install speedtest
```

## Usage

### Overview

```bash
speedtest <COMMAND>
```

### Commands

#### `serve`

```bash
speedtest serve <SOCKET>
```

Set up a server on one computer to receive network speed tests from another computer.

##### Arguments

- `<SOCKET>`: Socket address for the server.

##### Options

- `-h, --help`: Print help.

#### `test`

```bash
speedtest test [OPTIONS] <SOCKET>
```

Perform a network speed test from one computer to a server on another computer.

##### Arguments

- `<SOCKET>`: Socket address for the server.

##### Options

- `-l, --length <LENGTH>`: Specify the amount of data to send to the server in megabytes (default: 10).
- `-h, --help`: Print help.

### Global Options

- `-h, --help`: Print help.
- `-V, --version`: Print version.

## Examples

1. Set up a server on one computer:

```bash
speedtest serve 127.0.0.1:8080
```

2. Run a speed test from another computer:

```bash
speedtest test 127.0.0.1:8080
```
