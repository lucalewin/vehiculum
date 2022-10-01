pub mod frame;
pub mod message;
pub mod encryption;
pub mod flags;

use std::{net::TcpStream, io::{BufWriter, Write, BufReader, Read}};
use crate::{error::{Error, ErrorCode}, channel::Channel, messenger::{flags::{FrameType, EncryptionType}, message::{MessageType, Message}, frame::Frame}};

pub struct Messenger;

impl Messenger {
    pub fn send(stream: &TcpStream, frame: Frame) -> std::io::Result<()> {
        let mut writer = BufWriter::new(stream);

        writer.write(&frame.to_bytes()).unwrap();
        writer.flush().unwrap();

        Ok(())
    }

    pub fn receive(stream: &TcpStream) -> Result<Frame, Error> {
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
                todo!("decryption of encrypted payload");
            }

            payload.extend_from_slice(content);

            if payload.len() >= total_length as usize {
                break;
            }
        }

        let message = Message::new(MessageType::try_from([payload[0], payload[1]]).unwrap(), payload[2..].to_vec());

        Ok(Frame::new(channel, flags, message))
    }
}
