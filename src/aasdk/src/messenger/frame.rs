use crate::channel::Channel;

use super::message::Message;

#[derive(Debug)]
pub enum FrameSizeType {
    Short = 0,
    Extended = 1,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct FrameSize {
    frame_size_type: FrameSizeType,
    frame_size: usize,
    total_size: usize,
}

#[derive(Debug)]
pub struct Frame {
    channel: Channel,
    flags: u8,
    pub message: Message,
}

impl Frame {
    pub fn new(channel: Channel, flags: u8, message: Message) -> Self {
        Self { channel, flags, message }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.push(self.channel as u8);
        buffer.push(self.flags);
        buffer.extend_from_slice(self.message.to_bytes().as_slice());

        buffer
    }
}
