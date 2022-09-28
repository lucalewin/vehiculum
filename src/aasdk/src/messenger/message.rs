use bitflags::bitflags;

bitflags!{
    pub struct Channel: u8 {
        const CONTROL = 0;
        const INPUT = 1;
        const SENSOR = 2;
        const VIDEO = 3;
        const MEDIA_AUDIO = 4;
        const SPEECH_AUDIO = 5;
        const SYSTEM_AUDIO = 6;
        const AV_INPUT = 7;
        const BLUETOOTH = 8;
        const NONE = 255;
    }

    pub struct MessageTypeFlags: u8 {
        const CONTROL = 0;
        const SPECIFIC = 1 << 2;
    }
}

#[derive(Debug)]
pub struct MessageType(u16);

impl MessageType {
    pub const NONE: u16 = 0x0000;
    pub const VERSION_REQUEST: u16 = 0x0001;
    pub const VERSION_RESPONSE: u16 = 0x0002;
    pub const SSL_HANDSHAKE: u16 = 0x0003;
    pub const AUTH_COMPLETE: u16 = 0x0004;
    pub const SERVICE_DISCOVERY_REQUEST: u16 = 0x0005;
    pub const SERVICE_DISCOVERY_RESPONSE: u16 = 0x0006;
    pub const CHANNEL_OPEN_REQUEST: u16 = 0x0007;
    pub const CHANNEL_OPEN_RESPONSE: u16 = 0x0008;
    pub const PING_REQUEST: u16 = 0x000b;
    pub const PING_RESPONSE: u16 = 0x000c;
    pub const NAVIGATION_FOCUS_REQUEST: u16 = 0x000d;
    pub const NAVIGATION_FOCUS_RESPONSE: u16 = 0x000e;
    pub const SHUTDOWN_REQUEST: u16 = 0x000f;
    pub const SHUTDOWN_RESPONSE: u16 = 0x0010;
    pub const VOICE_SESSION_REQUEST: u16 = 0x0011;
    pub const AUDIO_FOCUS_REQUEST: u16 = 0x0012;
    pub const AUDIO_FOCUS_RESPONSE: u16 = 0x0013;
}

#[derive(Debug)]
pub struct Message {
    channel: Channel,
    flags: u8,
    message_type: u16,
    payload: Vec<u8>,
}

impl Message {
    pub fn new(channel: Channel, flags: u8, message_type: u16, payload: Vec<u8>) -> Self {
        Self { channel, flags, message_type, payload }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.push(self.channel.bits());
        bytes.push(self.flags);
        bytes.extend(u16::to_be_bytes((self.payload.len() + 2) as u16));
        bytes.extend(u16::to_be_bytes(self.message_type));
        bytes.extend(&self.payload);

        bytes
    }

    pub fn channel(&self) -> Channel {
        self.channel
    }

    pub fn flags(&self) -> u8 {
        self.flags
    }

    pub fn message_type(&self) -> u16 {
        self.message_type
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}
