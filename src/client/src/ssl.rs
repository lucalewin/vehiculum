use std::{fs::File, io::{BufReader, Read, Write}, path::Path};

use openssl::{ssl::{SslStream, SslOptions, Ssl, SslVerifyMode, SslFiletype, SslContext, SslMethod}, dh::Dh, pkey::Params, x509::X509StoreContextRef};

use log::trace;

pub struct SslHandler {
    // ssl_context: SslContext,
    pub ssl_stream: SslStream<BioStream>,
}

impl SslHandler {
    pub fn new() -> Self {
        let ssl_context = Self::init_ssl_context();
        let ssl_stream = Self::init_ssl(&ssl_context);

        Self {
            // ssl_context,
            ssl_stream
        }
    }

    fn init_ssl_context() -> SslContext {
        let mut ctx = SslContext::builder(SslMethod::tls_client()).unwrap();

        let cert = Path::new("cert/headunit.crt");
        let key = Path::new("cert/headunit.key");

        ctx.set_private_key_file(key, SslFiletype::PEM).expect("could net set private key");
        ctx.set_certificate_file(cert, SslFiletype::PEM).expect("could net set certificate");

        ctx.set_tmp_dh(&Self::load_dhparams()).expect("could not set temporary dh params");
        ctx.set_verify_callback(SslVerifyMode::PEER, Self::verify);
        ctx.set_options(SslOptions::NO_TLSV1_3);

        ctx.build()
    }

    fn init_ssl(ssl_context: &SslContext) -> SslStream<BioStream> {
        let mut ssl = Ssl::new(ssl_context).unwrap();

        ssl.set_connect_state();

        let bio_stream = BioStream::new();
        SslStream::new(ssl, bio_stream).unwrap()
    }

    fn load_dhparams() -> Dh<Params> {
        let file = File::open("cert/dhparams.pem").expect("cannot open dhparams file");
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        
        // Read file into vector.
        reader.read_to_end(&mut buffer).expect("cannot read dhparams file");

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

    pub fn bio_read(&mut self, mut buffer: &mut [u8]) -> std::io::Result<usize> {
        trace!("bio_read");
        let stream = self.ssl_stream.get_mut();
        let result = buffer.write(stream.write_bio.as_slice());

        stream.write_bio.clear();

        result
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
