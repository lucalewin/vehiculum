
use bitflags::bitflags;

bitflags! {
    pub struct VersionResponseStatus: u16 {
        const MATCH = 0x0000;
        const MISMATCH = 0xffff;
    }
}
