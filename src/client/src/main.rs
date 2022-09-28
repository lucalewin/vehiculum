use std::{net::TcpStream, io::{BufWriter, Write, BufReader, Read}};

use aasdk::{messenger::{message::{Message, Channel, MessageTypeFlags, MessageType}, frame::FrameType, encryption::EncryptionType}, proto::enums::VersionResponseStatus, error::{Error, ErrorCode}};

mod logging;
mod ssl;

use log::{debug, trace};
use ssl::SslHandler;

fn main() -> std::io::Result<()> {
    // setup logging
    logging::init(true);

    let stream = TcpStream::connect("localhost:5277")?;

    debug!("Connected to server...");

    Communicator::new(stream).setup()
}

struct Communicator {
    stream: TcpStream,
    ssl_handler: SslHandler,
}

impl Communicator {
    pub fn new(stream: TcpStream) -> Self {
        let ssl_handler = SslHandler::new();
        Communicator { stream, ssl_handler }
    }

    pub fn setup(&mut self) -> std::io::Result<()> {
        // version negotiation
        self.send_version_request()?;
        self.expect_version_response().unwrap();

        debug!("version negotiation succeeded");

        // ssl handshake
        let mut msg = None;
        while !self.do_ssl_handshake(&msg) {
            msg = Some(self.recv().unwrap());
        }

        debug!("ssl handshake completed");

        self.send_auth_complete()?;

        Ok(())
    }

    fn send_version_request(&mut self) -> std::io::Result<()> {
        // current version = 1.1
        let major = u16::to_be_bytes(0x0001);
        let minor = u16::to_be_bytes(0x0001);

        let message = Message::new(
            Channel::CONTROL,
            EncryptionType::PLAIN.bits() | FrameType::BULK.bits() | MessageTypeFlags::CONTROL.bits(),
            MessageType::VERSION_REQUEST,
            [major, minor].concat());

        self.send(message)
    }

    fn expect_version_response(&mut self) -> Result<(), Error> {
        let message = self.recv().unwrap();

        if message.payload().len() != 6 ||
                message.message_type() != MessageType::VERSION_RESPONSE ||
                u16::from_be_bytes([message.payload()[4], message.payload()[5]]) != VersionResponseStatus::MATCH.bits() {
            return Err(Error::new(ErrorCode::ControlVersionResponse, 0, "did not get version response (or version did not match)".to_string()));
        }

        Ok(())
    }

    fn do_ssl_handshake(&mut self, message: &Option<Message>) -> bool {
        // self.ssl_handler.reinit_ssl();
        
        if let Some(message) = message {
            if message.message_type() != MessageType::SSL_HANDSHAKE {
                panic!("expected SSL handshake message");
            }

            trace!("some message");

            self.ssl_handler.bio_write(message.payload());
        }

        trace!("SSL_connect");

        match self.ssl_handler.ssl_stream.connect() {
            Ok(_) => return true,
            Err(e) => {
                if e.code() != openssl::ssl::ErrorCode::WANT_READ {
                    panic!("SSL_connect failed: {:?}", e);
                }
            }
        };

        trace!("WANT_READ");

        let mut message_buffer = Vec::new();
        
        let mut buffer = [0u8; 512];
        // while let Ok(len) = bio_stream.read(&mut buffer) {
        while let Ok(len) = self.ssl_handler.bio_read(&mut buffer) {
            if len <= 0 { break }

            message_buffer.extend_from_slice(&buffer[..len]);
        }
        
        // create response handshake message
        let message = Message::new(
                Channel::CONTROL,
                EncryptionType::PLAIN.bits() | FrameType::BULK.bits(),
                MessageType::SSL_HANDSHAKE,
                message_buffer);

        debug!("SSL send message: {:?}", message);

        self.send(message).unwrap();

        return false;
    }

    fn send_auth_complete(&mut self) -> std::io::Result<()> {
        let message = Message::new(
                Channel::CONTROL,
                FrameType::BULK.bits() | EncryptionType::PLAIN.bits() | MessageTypeFlags::CONTROL.bits(),
                MessageType::AUTH_COMPLETE,
                vec![0x08, 0x00]);

        self.send(message)
    }

    fn send(&mut self, message: Message) -> std::io::Result<()> {
        let mut writer = BufWriter::new(&self.stream);

        writer.write(&message.to_bytes())?;
        writer.flush()?;

        Ok(())
    }

    fn recv(&mut self) -> Result<Message, Error> {
        let mut reader = BufReader::new(&self.stream);

        const BUFFER_SIZE: usize = 100_000;
        let mut buffer = vec![0u8; BUFFER_SIZE];

        let mut channel: Channel;
        let mut flags: u8;
        let mut message_type: u16;
        let mut payload = Vec::new();

        let mut total_length = 0;

        loop {
            let transferred = reader.read(&mut buffer[..]).unwrap();

            // debug!("recieved message: {:?}", &buffer[..transferred]);

            channel = Channel::from_bits(buffer[0]).expect("not a valid channel");
            flags = buffer[1];

            let length = u16::from_be_bytes([buffer[2], buffer[3]]);
            let mut offset = 4;

            if flags & FrameType::BULK.bits() == FrameType::FIRST.bits() {
                total_length = u32::from_be_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);
                offset += 4;
            }
            
            let content = &buffer[offset..transferred];

            message_type = u16::from_be_bytes([content[0], content[1]]);

            if length as usize != content.len() {
                return Err(Error::new(ErrorCode::InvalidPayloadLength, 0,
                    format!("Invalid payload length: expected {} bytes, got {}", length, content.len())));
            }

            // check if payload is encrypted
            if flags & EncryptionType::ENCRYPTED.bits() == EncryptionType::ENCRYPTED.bits() {
                todo!()
            }

            // starting from index 2 because sizeof(MessageType) = 2
            payload.extend_from_slice(&content[2..]);

            if payload.len() >= total_length as usize {
                break;
            }
        }

        Ok(Message::new(channel, flags, message_type, payload))
    }
}
