use bitflags::bitflags;

bitflags! {
    pub struct VersionResponseStatus: u16 {
        const MATCH = 0x0000;
        const NOT_MATCH = 0xFFFF;
    }
}
