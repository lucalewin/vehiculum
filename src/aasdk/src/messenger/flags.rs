use bitflags::bitflags;

bitflags!{
    pub struct EncryptionType: u8 {
        const PLAIN = 0;
        const ENCRYPTED = 1 << 3;
    }

    pub struct FrameType: u8 {
        const MIDDLE = 0;
        const FIRST = 1 << 0;
        const LAST = 1 << 1;
        const BULK = FrameType::FIRST.bits | FrameType::LAST.bits;
    }

    pub struct MessageTypeFlags: u8 {
        const CONTROL = 0;
        const SPECIFIC = 1 << 2;
    }
}
