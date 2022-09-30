mod aasdk;
mod communication;
mod logging;
mod ssl;

use log::{info, debug};

use std::net::{TcpListener, TcpStream};
use common::constants;
use communication::Communicator;

fn main() -> std::io::Result<()> {
    // setup logging
    logging::init(true);

    info!("Starting TCP server on port {} ...", constants::HEAD_UNIT_PORT);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", constants::HEAD_UNIT_PORT))?;

    info!("Successfully started server on port {}", constants::HEAD_UNIT_PORT);

    debug!("waiting for clients to connect");

    // accept connections and process them serially
    for incoming in listener.incoming() {
        let stream = incoming?;
        debug!("incoming connection: {:?}", stream);
        handle_client(stream)?;
    }

    Ok(())
}

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    Communicator::new(stream).start();
    Ok(())
}

//        0        3        0        6        0        1        0        1        0        2
// 00000000 00000011 00000000 00000110 00000000 00000001 00000000 00000001 00000000 00000010
