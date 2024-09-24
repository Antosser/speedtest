mod client;
mod server;

use client::client;
use color_eyre::eyre::eyre;
use color_eyre::eyre::WrapErr; // Needed for `.context()`
use server::server;
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use std::net::SocketAddrV4;

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

fn main() -> color_eyre::Result<()> {
    color_eyre::install().context("Failed to install color_eyre")?;
    tracing_subscriber::registry()
        .with(ErrorLayer::default())
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();
    let args = Args::parse();

    match args.mode {
        Mode::Server {
            socket: socket_addr,
        } => server(&socket_addr).context("Server mode failed to start")?,
        Mode::Client {
            socket: socket_addr,
            length,
        } => client(
            &socket_addr,
            parse_size::parse_size(length)
                .map_err(|e| eyre!(e))
                .context("Failed to parse the length argument")?,
        )
        .context("Client mode failed to run")?,
    }

    Ok(())
}
