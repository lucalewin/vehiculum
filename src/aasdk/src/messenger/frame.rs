use bitflags::bitflags;

bitflags! {
    pub struct FrameType: u8 {
        const MIDDLE = 0;
        const FIRST = 1 << 0;
        const LAST = 1 << 1;
        const BULK = FrameType::FIRST.bits | FrameType::LAST.bits;
    }

    pub struct FrameSizeType: u8 {
        const SHORT = 0;
        const EXTENDED = 1;
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct FrameSize {
    frame_size_type: FrameSizeType,
    frame_size: usize,
    total_size: usize,
}

pub struct FrameHeader {
    
}
