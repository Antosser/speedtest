use std::{
    io::Write,
    net::{SocketAddrV4, TcpStream},
    time::Instant,
};

use colored::Colorize;
use humansize::{format_size, BINARY, DECIMAL};
use itertools::Itertools;
use rand::Rng;
use tracing::info;

pub fn client(socket_addr: &SocketAddrV4, length: u64) -> anyhow::Result<()> {
    const BUFFER_SIZE: u64 = 1_000_000;

    let mut stream = TcpStream::connect(socket_addr)?;
    info!("Stream accepted");

    let mut rng = rand::thread_rng();
    let buffer = (0..BUFFER_SIZE).map(|_| rng.gen::<u8>()).collect_vec();
    info!("Writing data...");
    let start_time = Instant::now();

    let mut remaining = length;
    while remaining > 0 {
        if remaining > BUFFER_SIZE {
            stream.write_all(&buffer)?;
            remaining -= BUFFER_SIZE;
        } else {
            stream.write_all(&buffer[0usize..remaining as usize])?;
            remaining = 0;
        }
    }
    let elapsed_time = start_time.elapsed();
    let bytes_per_second = (length as f64 / elapsed_time.as_secs_f64()) as u64;
    println!(
        "Transferred data: {}, {}",
        format_size(length, DECIMAL).cyan(),
        format_size(length, BINARY).magenta()
    );
    println!("Elapsed time: {}", format!("{:?}", elapsed_time).cyan());
    println!(
        "Transfer speed: {}, {}",
        format!("{}/s", format_size(bytes_per_second, DECIMAL)).cyan(),
        format!("{}/s", format_size(bytes_per_second, BINARY)).magenta(),
    );

    Ok(())
}
