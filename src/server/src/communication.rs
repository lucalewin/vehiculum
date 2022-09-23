use crate::aasdk;

use std::io::{BufReader, Read, Write, BufWriter};
use std::net::TcpStream;
use aasdk::channel::ChannelID;
use common::constants;
use aasdk::messenger::{Message, MessageType, MessageTypeFlags, encryption::EncryptionType, frame::FrameType};



pub struct Communicator {
    stream: TcpStream
}

impl Communicator {
    pub fn new(stream: TcpStream) -> Communicator {
        Self { stream }
    }

    pub fn init(&mut self) -> std::io::Result<()> {

        let version_request_msg = self.receive_message();

        println!("version_request_msg {:?}", version_request_msg);

        let major = u16::to_be_bytes(constants::AASDK_MAJOR);
        let minor = u16::to_be_bytes(constants::AASDK_MINOR);

        let msg = Message::new(
            ChannelID::Control,
            EncryptionType::Plain as u8 | FrameType::Bulk as u8 | MessageTypeFlags::Control as u8,
            MessageType::VersionResponse,
            [major, minor].concat());


        println!("sending version response: {:?}", msg);

        self.send_message(msg).unwrap();

        println!("message sent");

        let message = self.receive_message().unwrap();

        println!("message received: {:?}", &message);

        self.do_ssh_handshake(message).unwrap();

        

        Ok(())
    }

    fn do_ssh_handshake(&mut self, message: Message) -> Result<(), std::io::Error> {
        if message.message_type != MessageType::SslHandshake {
            panic!("expected SslHandshake message, but received {:?}", message.message_type);
        }
        println!("test {:?}", message);
        println!("sending handshake");
        Ok(())
    }

    pub fn send_message(&mut self, message: Message) -> Result<(), aasdk::error::Error> {
        let mut writer = BufWriter::new(&self.stream);
        let buffer = message.to_bytes();

        println!("buffer = {:?}", buffer);

        writer.write_all(&buffer).unwrap();
        writer.flush().unwrap();

        println!("flushed");

        Ok(())
    }

    pub fn receive_message(&mut self) -> Result<Message, aasdk::error::Error> {
        println!("receiving message");

        let mut reader = BufReader::new(&self.stream);

        let mut channel = 0;
        let mut flags = 0;
        let mut full_content = Vec::<u8>::new();

        loop {
            let mut buffer = vec![0u8; 4];

            reader.read_exact(&mut buffer).unwrap();

            channel = buffer[0];
            flags = buffer[1];

            let length = u16::from_be_bytes([buffer[2], buffer[3]]);

            println!("channel = {}, flags = {}, length = {}", channel, flags, length);

            let mut total_length = 0;

            if (flags & FrameType::Bulk as u8) == FrameType::First as u8 {
                let mut buffer = [0u8; 4];

                reader.read_exact(&mut buffer).unwrap();
                
                total_length = u32::from_be_bytes(buffer);
            }

            let mut content = vec![0u8; length as usize];

            reader.read_exact(&mut content).unwrap();

            println!("content = {:?}, size = {}", content, content.len());

            if flags & EncryptionType::Encrypted as u8 == EncryptionType::Encrypted as u8 {
                todo!("encrypt message");
            }

            full_content.extend_from_slice(&content);

            if full_content.len() >= total_length as usize {
                break;
            }
        }

        Ok(Message::from_primitives(channel, flags, u16::from_be_bytes([full_content[0], full_content[1]]), full_content[2..].to_vec()))
    }
}
