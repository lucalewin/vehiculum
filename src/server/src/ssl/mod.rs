pub mod bio;

use std::{path::Path, io::{Read, Write, BufReader}, fs::File};

use openssl::{ssl::{SslContext, SslMethod, SslFiletype, SslVerifyMode, Ssl, SslStream, SslOptions}, x509::X509StoreContextRef, dh::Dh, pkey::Params};
use log::{debug, trace};

use crate::aasdk::{messenger::{Message, MessageType, encryption::EncryptionType, frame::FrameType}, channel::ChannelID};


pub struct SslHandler {
    pub ssl_stream: SslStream<BioStream>,
    // bio_write: BufWriter<&'a BioStream>,
    // bio_read: BufReader<&'a BioStream>,
}

impl SslHandler {
    pub fn init() -> Self {
        // setup ssl context
        let mut ctx = SslContext::builder(SslMethod::tls_server()).unwrap();

        let cert = Path::new("cert/android_auto.crt");
        let key = Path::new("cert/android_auto.key");
    
        ctx.set_cipher_list("DEFAULT").unwrap();
        ctx.set_private_key_file(key, SslFiletype::PEM).unwrap();
        ctx.set_certificate_file(cert, SslFiletype::PEM).unwrap();

        ctx.set_tmp_dh(&Self::load_dhparams()).unwrap();
        ctx.set_verify_callback(SslVerifyMode::PEER, Self::verify);
        ctx.set_options(SslOptions::NO_TLSV1_3);

        let ssl_context = ctx.build();
        let mut ssl = Ssl::new(&ssl_context).unwrap();

        ssl.set_accept_state();
        
        let bio_stream = BioStream::new();
        let ssl_stream = SslStream::new(ssl, bio_stream).unwrap();

        Self { ssl_stream }
    }

    fn load_dhparams() -> Dh<Params> {
        let file = File::open("cert/dhparams.pem").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        
        // Read file into vector.
        reader.read_to_end(&mut buffer).unwrap();

        Dh::params_from_pem(buffer.as_slice()).unwrap()
    }

    fn verify<'r>(_b: bool, _store: &'r mut X509StoreContextRef) -> bool {
        true
    }

    pub fn bio_write(&mut self, buffer: &[u8]) {
        trace!("bio_write");
        let stream = self.ssl_stream.get_mut();

        // write to read_bio
        stream.read_bio.clear();
        stream.read_bio.write_all(buffer).unwrap();
    }

    pub fn bio_read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        trace!("bio_read");
        let stream = self.ssl_stream.get_mut();

        {
            if buffer.len() == stream.write_bio.len() {
                buffer.copy_from_slice(&stream.write_bio);
                trace!("read: copy_from_vec#1: {:?}", buffer.len());
                stream.write_bio.clear();
                Ok(buffer.len())
            } else if buffer.len() > stream.write_bio.len() {
                buffer[..stream.write_bio.len()].copy_from_slice(&stream.write_bio);
                trace!("read: copy_from_vec#2: {:?}", stream.write_bio.len());
                stream.write_bio.clear();
                Ok(stream.write_bio.len())
            } else {
                buffer.copy_from_slice(&stream.write_bio[..buffer.len()]);
                trace!("read: copy_from_vec#3: {:?}:{:?}", buffer.len(), stream.write_bio.len());
                stream.write_bio.drain(0..buffer.len());
                Ok(buffer.len())
            }
        }

    }

    pub fn decrypt_message(&self, _message: Vec<u8>) -> std::io::Result<Vec<u8>> {
        Ok(vec![])
    }

}

#[derive(Debug)]
pub struct BioStream {
    pub read_bio: Vec<u8>,
    pub write_bio: Vec<u8>
}

impl BioStream {
    pub fn new() -> Self {
        Self {
            read_bio: Vec::new(),
            write_bio: Vec::new()
        }
    }
}

impl Read for BioStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {

        trace!("read");

        if self.read_bio.is_empty() {
            return Err(std::io::Error::new(std::io::ErrorKind::WouldBlock, "bio_in is empty"));
        }

        {
            if buf.len() == self.read_bio.len() {
                buf.copy_from_slice(&self.read_bio);
                trace!("read: copy_from_vec#1: {:?}", buf.len());
                self.read_bio.clear();
                Ok(buf.len())
            } else if buf.len() > self.read_bio.len() {
                buf[..self.read_bio.len()].copy_from_slice(&self.read_bio);
                trace!("read: copy_from_vec#2: {:?}", self.read_bio.len());
                self.read_bio.clear();
                Ok(self.read_bio.len())
            } else {
                buf.copy_from_slice(&self.read_bio[..buf.len()]);
                trace!("read: copy_from_vec#3: {:?}:{:?}", buf.len(), self.read_bio.len());
                self.read_bio.drain(0..buf.len());
                Ok(buf.len())
            }
        }
    }
}

impl Write for BioStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        trace!("write");
        self.write_bio.clear();
        self.write_bio.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        trace!("flush");
        Ok(())
    }
}
