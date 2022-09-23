mod aasdk;
mod communication;

use std::net::{TcpListener, TcpStream};
use common::constants;
use communication::Communicator;

fn main() -> std::io::Result<()> {
    println!("Desktop Head Unit Port: {}", constants::HEAD_UNIT_PORT);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", constants::HEAD_UNIT_PORT))?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("incoming {:?}", stream);
        handle_client(stream?);
    }

    Ok(())
}

fn handle_client(stream: TcpStream) {

    let mut communicator = Communicator::new(stream);

    communicator.init();
}

//        0        3        0        6        0        1        0        1        0        2
// 00000000 00000011 00000000 00000110 00000000 00000001 00000000 00000001 00000000 00000010
