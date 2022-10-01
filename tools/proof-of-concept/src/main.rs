use std::net::{TcpListener, TcpStream};

use aasdk::messenger::Messenger;
use aasdk::messenger::message::MessageType;

fn main() {
    let dhu_listener = TcpListener::bind("127.0.0.1:5277").unwrap();

    println!("listening on port {:?}", dhu_listener.local_addr().unwrap());

    for incoming in dhu_listener.incoming() {
        let stream = incoming.unwrap();
        println!("Client connected: {:?}", stream);
        handle_dhu(stream);
        return;
    }
}

fn handle_dhu(dhu_stream: TcpStream) {
    println!("DHU connected: {:?}", dhu_stream);
    let client_stream = TcpStream::connect("127.0.0.1:6277").unwrap();

    loop {
        let dhu_request = Messenger::receive(&dhu_stream).unwrap();

        match dhu_request.message.message_type() {
            MessageType::AuthComplete => {
                if matches!(dhu_request.message.payload(), &[8,0]) {
                    println!(" * Successfully authenticated!!!");
                    return;
                } else {
                    panic!("unsuccessful authentication");
                }
            }
            _ => ()
        }

        Messenger::send(&client_stream, dhu_request).unwrap();

        let client_response = Messenger::receive(&client_stream).unwrap();
        Messenger::send(&dhu_stream, client_response).unwrap();
    }
}
