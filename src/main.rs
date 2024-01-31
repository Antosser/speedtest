use std::{
    io::{Read, Write},
    net::{SocketAddrV4, TcpListener, TcpStream},
    thread,
    time::Instant,
};

use clap::{Parser, Subcommand};
use humansize::{format_size, DECIMAL};
use itertools::Itertools;
use tracing::{error, info, instrument};

#[derive(Subcommand, Clone, PartialEq, Debug)]
enum Mode {
    #[command(name = "serve")]
    Server {
        /// Socket address
        socket: SocketAddrV4,
    },
    #[command(name = "test")]
    Client {
        /// Socket address
        socket: SocketAddrV4,

        /// How much bytes to send to the server in MEGABYTES
        #[arg(short, long, default_value_t = 10)]
        length: u64,
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

#[instrument]
fn server(socket_addr: &SocketAddrV4) -> anyhow::Result<()> {
    let listener = TcpListener::bind(socket_addr)?;
    info!("Listening...");
    for stream in listener.incoming() {
        thread::spawn(|| {
            if let Err(e) = || -> anyhow::Result<()> {
                let mut stream = stream?;
                info!("Incoming connection");
                stream.read_to_end(&mut Vec::new())?;

                Ok(())
            }() {
                error!("Error in thread: {}", e);
            }
        });
    }

    Ok(())
}

#[instrument]
fn client(socket_addr: &SocketAddrV4, mut length: u64) -> anyhow::Result<()> {
    length *= 1_000_000;

    let mut stream = TcpStream::connect(socket_addr)?;
    info!("Stream accepted");

    let buffer = (0..255).cycle().take(10_000).collect_vec();
    info!("Writing data...");
    let start_time = Instant::now();
    for _ in (0..length).step_by(buffer.len()) {
        stream.write_all(&buffer)?;
    }
    let elapsed_time = start_time.elapsed();
    println!("Transferred data: {}", format_size(length, DECIMAL));
    println!("Elapsed time: {:?}", elapsed_time);
    println!(
        "Transfer speed: {}/s",
        format_size((length as f64 / elapsed_time.as_secs_f64()) as u64, DECIMAL)
    );

    Ok(())
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
        } => client(&socket_addr, length)?,
    }

    Ok(())
}
