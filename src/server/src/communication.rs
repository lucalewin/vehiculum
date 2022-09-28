use crate::aasdk;
use crate::aasdk::messenger::MessageTypeFlags;
use crate::aasdk::proto::VersionResponseStatus;
use crate::aasdk::channel::ChannelHandler;
use crate::ssl::SslHandler;

use std::io::{BufReader, Read, Write, BufWriter};
use std::net::TcpStream;
use aasdk::channel::ChannelID;
use common::constants;
use aasdk::messenger::{Message, MessageType, encryption::EncryptionType, frame::FrameType};

use log::{debug};

pub struct Communicator {
    stream: TcpStream,
    ssl_handler: SslHandler,
    channel_handler: ChannelHandler
}

impl Communicator {
    pub fn new(stream: TcpStream) -> Self {
        let ssl_handler = SslHandler::init();
        let channel_handler = ChannelHandler::new();

        Self { stream, ssl_handler, channel_handler }
    }

    pub fn start(mut self) {
        std::thread::spawn(move || {
            loop {
                let message = self.receive_message().unwrap();
                self.handle_message(message).unwrap();
            }
        });
    }

    pub fn handle_message(&mut self, message: Message) -> std::io::Result<()> {
        debug!("handling message: {:?}", message.message_type);

        if message.channel != ChannelID::Control {
            return self.channel_handler.handle_message(message);
        }

        match message.message_type {
            MessageType::AudioFocusResponse | MessageType::NavigationFocusResponse => self.channel_handler.handle_message(message),
            MessageType::VersionRequest => self.handle_version_request(message),
            MessageType::SslHandshake => self.handle_ssl_handshake(message),
            MessageType::AuthComplete => {
                debug!("message: {:?}", message);
                self.send_service_discovery_request()},
            MessageType::ServiceDiscoveryResponse => self.handle_service_discovery_response(),
            MessageType::PingRequest => self.handle_ping_request(),
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "UnknownMessage"))
        }
    }

    fn handle_version_request(&mut self, message: Message) -> std::io::Result<()> {
        let major_version = u16::from_be_bytes([message.content[0], message.content[1]]);
        let minor_version = u16::from_be_bytes([message.content[2], message.content[3]]);

        debug!("Head Unit Version: {}.{}", major_version, minor_version);

        match major_version {
            1 => self.send_version_response(),
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "unsupported version of android auto"))
        }
    }

    fn send_version_response(&mut self) -> std::io::Result<()> {
        let major = u16::to_be_bytes(constants::AASDK_MAJOR);
        let minor = u16::to_be_bytes(constants::AASDK_MINOR);
        let response_status = u16::to_be_bytes(VersionResponseStatus::MATCH.bits());

        let message = Message::new(
            ChannelID::Control,
            EncryptionType::Plain as u8 | FrameType::Bulk as u8 | MessageTypeFlags::Control as u8,
            MessageType::VersionResponse,
            [major, minor, response_status].concat());

        self.send_message(message)
    }

    fn send_service_discovery_request(&mut self) -> std::io::Result<()> {
        todo!()
    }

    fn handle_service_discovery_response(&mut self) -> std::io::Result<()> {
        todo!()
    }

    fn handle_ping_request(&mut self) -> std::io::Result<()> {
        todo!()
    }

    fn handle_ssl_handshake(&mut self, message: Message) -> std::io::Result<()> {
        if message.message_type != MessageType::SslHandshake {
            return Err(std::io::Error::new(std::io::ErrorKind::Other,
                format!("expected {:?} message, but received {:?}", MessageType::SslHandshake, message.message_type)));
        }

        self.ssl_handler.bio_write(message.content.as_slice())?;

        match self.ssl_handler.ssl_stream.accept() {
            Ok(_) => return Ok(()),
            Err(e) => {
                if e.code() != openssl::ssl::ErrorCode::WANT_READ {
                    panic!("SSL_connect failed: {:?}", e);
                }
            }
        }

        debug!("handshake accepted");

        // create buffer with 512 bytes
        let mut message_buffer = Vec::new();
        
        let mut buffer = [0u8; 512];
        while let Ok(len) = self.ssl_handler.bio_read(&mut buffer) {
            if len <= 0 { break }

            message_buffer.extend_from_slice(&buffer[..len]);
        }
        
        debug!("MESSAGE: {:?}", message_buffer);

        // create response handshake message
        let message = Message::new(
                ChannelID::Control,
                EncryptionType::Plain as u8 | FrameType::Bulk as u8,
                MessageType::SslHandshake,
                message_buffer);

        debug!("send message: {:?}", message);

        self.send_message(message)?;

        Ok(())
    }

    pub fn send_message(&mut self, message: Message) -> std::io::Result<()> {
        let mut writer = BufWriter::new(&self.stream);
        let buffer = message.to_bytes();

        writer.write_all(&buffer)?;
        writer.flush()?;

        Ok(())
    }

    pub fn receive_message(&mut self) -> std::io::Result<Message> {
        let mut reader = BufReader::new(&self.stream);

        let mut channel;
        let mut flags;
        let mut full_content = Vec::<u8>::new();

        loop {
            let mut buffer = vec![0u8; 4];
            reader.read_exact(&mut buffer)?;

            channel = buffer[0];
            flags = buffer[1];

            let length = u16::from_be_bytes([buffer[2], buffer[3]]);

            debug!("recieved message: channel = {}, flags = {}, length = {}", channel, flags, length);

            let mut total_length = 0;

            if (flags & FrameType::Bulk as u8) == FrameType::First as u8 {
                let mut buffer = [0u8; 4];
                reader.read_exact(&mut buffer)?;
                total_length = u32::from_be_bytes(buffer);
            }

            let mut content = vec![0u8; length as usize];
            reader.read_exact(&mut content)?;

            if flags & EncryptionType::Encrypted as u8 == EncryptionType::Encrypted as u8 {
                content = self.ssl_handler.decrypt_message(content)?;
            }

            full_content.extend_from_slice(&content);

            if full_content.len() >= total_length as usize {
                break;
            }
        }

        Ok(Message::from_primitives(channel, flags, u16::from_be_bytes([full_content[0], full_content[1]]), full_content[2..].to_vec()))
    }
}
