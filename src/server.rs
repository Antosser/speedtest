use std::{
    io::Read,
    net::{SocketAddrV4, TcpListener},
    thread,
};

use tracing::{error, info, instrument};

#[instrument]
pub fn server(socket_addr: &SocketAddrV4) -> anyhow::Result<()> {
    let listener = TcpListener::bind(socket_addr)?;
    info!("Listening...");
    for stream in listener.incoming() {
        thread::spawn(|| {
            if let Err(e) = || -> anyhow::Result<()> {
                let mut stream = stream?;
                info!("Incoming connection");
                let mut buffer = Box::new([0u8; 100_000]);
                while stream.read(&mut *buffer)? != 0 {}

                Ok(())
            }() {
                error!("Error in thread: {}", e);
            }
        });
    }

    Ok(())
}
