use bitflags::bitflags;

use crate::error::Error;

bitflags!{
    pub struct EncryptionType: u8 {
        const PLAIN = 0;
        const ENCRYPTED = 1 << 3;
    }
}

pub trait ICryptor {
    fn init(&self);
    fn do_handshake(&self) -> Result<bool, Error>;

    fn encrypt(&self, output: &mut [u8], data: &[u8]) -> Result<usize, Error>;
    fn decrypt(&self, output: &mut [u8], data: &[u8]) -> Result<usize, Error>;

    fn read_handshake_buffer(&self) -> Result<Vec<u8>, Error>;
    fn write_handshake_buffer(&self, data: &[u8]) -> Result<usize, Error>;
}
