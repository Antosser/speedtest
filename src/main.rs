mod client;
mod server;

use client::client;
use server::server;

use std::net::SocketAddrV4;

use anyhow::anyhow;
use clap::{Parser, Subcommand};

#[derive(Subcommand, Clone, PartialEq, Debug)]
enum Mode {
    /// Listen to incoming TCP connections on the given socket address
    #[command(name = "serve")]
    Server {
        /// Socket address
        socket: SocketAddrV4,
    },

    /// Connect to a TCP server with the given socket address
    #[command(name = "test")]
    Client {
        /// Socket address
        socket: SocketAddrV4,

        /// How many bytes to send to the server
        #[arg(short, long, default_value = "10MB")]
        length: String,
    },
}

/// Rust-based network speed testing tool between two computers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Mode in which the program operates
    #[command(subcommand)]
    mode: Mode,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    let args = Args::parse();

    match args.mode {
        Mode::Server {
            socket: socket_addr,
        } => server(&socket_addr)?,
        Mode::Client {
            socket: socket_addr,
            length,
        } => client(
            &socket_addr,
            parse_size::parse_size(length).map_err(|e| anyhow!(e))?,
        )?,
    }

    Ok(())
}
