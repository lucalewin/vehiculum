use std::net::TcpStream;

mod logging;
mod ssl;

use aasdk::{messenger::{message::{Message, MessageType}, flags::{EncryptionType, FrameType, MessageTypeFlags}, frame::Frame, Messenger}, channel::Channel, error::{Error, ErrorCode}, proto::enums::VersionResponseStatus};
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
            msg = Some(Messenger::receive(&self.stream).unwrap().message);
        }

        debug!("ssl handshake completed");

        self.send_auth_complete()?;

        let frame = Messenger::receive(&self.stream).unwrap();

        debug!("received frame: {:?}", frame);

        Ok(())
    }

    fn send_version_request(&mut self) -> std::io::Result<()> {
        // current version = 1.1
        const MAJOR: [u8; 2] = u16::to_be_bytes(0x0001);
        const MINOR: [u8; 2] = u16::to_be_bytes(0x0001);

        let frame = Frame::new(
            Channel::Control,
            EncryptionType::PLAIN.bits() | FrameType::BULK.bits() | MessageTypeFlags::CONTROL.bits(),
            Message::new(MessageType::VersionRequest, [MAJOR, MINOR].concat()));

        Messenger::send(&self.stream, frame)
    }

    fn expect_version_response(&mut self) -> Result<(), Error> {
        let message = Messenger::receive(&self.stream).unwrap().message;

        if message.payload().len() != 6 ||
                message.message_type() != MessageType::VersionResponse ||
                u16::from_be_bytes([message.payload()[4], message.payload()[5]]) != VersionResponseStatus::MATCH.bits() {
            return Err(Error::new(ErrorCode::ControlVersionResponse, 0, "did not get version response (or version did not match)".to_string()));
        }

        Ok(())
    }

    fn do_ssl_handshake(&mut self, message: &Option<Message>) -> bool {
        if let Some(message) = message {
            if message.message_type() != MessageType::SslHandshake {
                panic!("expected SSL handshake message");
            }

            self.ssl_handler.bio_write(message.payload());
        }

        trace!("SSL_connect");

        match self.ssl_handler.ssl_stream.connect() {
            Ok(_) => return true,
            Err(e) => {
                if e.code() != openssl::ssl::ErrorCode::WANT_READ {
                    panic!("SSL_connect failed: {:?}", e);
                }
                trace!("WANT_READ");
            }
        };

        let mut message_buffer = Vec::new();
        
        let mut buffer = [0u8; 512];
        while let Ok(len) = self.ssl_handler.bio_read(&mut buffer) {
            if len <= 0 { break; }
            message_buffer.extend_from_slice(&buffer[..len]);
        }
        
        // create response handshake message
        let frame = Frame::new(
                Channel::Control,
                EncryptionType::PLAIN.bits() | FrameType::BULK.bits(),
                Message::new(MessageType::SslHandshake, message_buffer));

        Messenger::send(&self.stream, frame).unwrap();

        return false;
    }

    fn send_auth_complete(&mut self) -> std::io::Result<()> {
        let frame = Frame::new(
            Channel::Control,
            FrameType::BULK.bits() | EncryptionType::PLAIN.bits() | MessageTypeFlags::CONTROL.bits(),
            Message::new(MessageType::AuthComplete, [0x08, 0x00].to_vec()));

        Messenger::send(&self.stream, frame)
    }
}
