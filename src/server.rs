use std::{
    io::Read,
    net::{SocketAddrV4, TcpListener},
    thread,
};

use color_eyre::eyre::WrapErr;
use tracing::{error, info, instrument}; // Import for `.context()`

#[instrument]
pub fn server(socket_addr: &SocketAddrV4) -> color_eyre::Result<()> {
    let listener = TcpListener::bind(socket_addr)
        .context("Failed to bind TcpListener to the socket address")?;
    info!("Listening...");
    for stream in listener.incoming() {
        thread::spawn(|| {
            if let Err(e) = || -> color_eyre::Result<()> {
                let mut stream = stream.context("Failed to accept incoming stream")?;
                info!("Incoming connection");
                let mut buffer = (0..1_000_000).map(|_| 0u8).collect::<Box<_>>();
                while stream
                    .read(&mut buffer)
                    .context("Failed to read from stream")?
                    != 0
                {}

                Ok(())
            }() {
                error!("Error in thread: {}", e);
            }
        });
    }

    Ok(())
}
