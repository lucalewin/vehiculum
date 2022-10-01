use std::{net::{TcpListener, TcpStream}, io::{BufWriter, Write, BufReader, Read}};

use aasdk::{messenger::frame::Frame, channel::Channel, error::{Error, ErrorCode}};
use aasdk::messenger::message::Message;
use aasdk::messenger::message::MessageType;
use aasdk::messenger::flags::*;

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
        let dhu_request = recv(&dhu_stream).unwrap();

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

        send(&client_stream, dhu_request).unwrap();

        let client_response = recv(&client_stream).unwrap();
        send(&dhu_stream, client_response).unwrap();
    }
}

fn send(stream: &TcpStream, frame: Frame) -> std::io::Result<()> {
    let mut writer = BufWriter::new(stream);

    writer.write(&frame.to_bytes())?;
    writer.flush()?;

    Ok(())
}

fn recv(stream: &TcpStream) -> Result<Frame, Error> {
    let mut reader = BufReader::new(stream);

    const BUFFER_SIZE: usize = 100_000;
    let mut buffer = vec![0u8; BUFFER_SIZE];

    let mut channel: Channel;
    let mut flags: u8;
    let mut payload = Vec::new();

    let mut total_length = 0;

    loop {
        let transferred = reader.read(&mut buffer[..]).unwrap();

        channel = Channel::try_from(buffer[0]).unwrap();
        flags = buffer[1];

        let length = u16::from_be_bytes([buffer[2], buffer[3]]);
        let mut offset = 4;

        if flags & FrameType::BULK.bits() == FrameType::FIRST.bits() {
            total_length = u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
            offset += 4;
        }
        
        let content = &buffer[offset..transferred];

        if length as usize != content.len() {
            return Err(Error::new(ErrorCode::InvalidPayloadLength, 0,
                format!("Invalid payload length: expected {} bytes, got {}", length, content.len())));
        }

        // check if payload is encrypted
        if flags & EncryptionType::ENCRYPTED.bits() == EncryptionType::ENCRYPTED.bits() {
            return Err(Error::new(ErrorCode::None, 0, "received encrypted payload".into()));
        }

        payload.extend_from_slice(content);

        if payload.len() >= total_length as usize {
            break;
        }
    }

    let message = Message::new(MessageType::try_from([payload[0], payload[1]]).unwrap(), payload[2..].to_vec());

    if message.message_type() == MessageType::AuthComplete {

    }

    Ok(Frame::new(channel, flags, message))
}
