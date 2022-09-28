use super::channel::ChannelID;

pub mod frame;
pub mod encryption;
pub mod flags;

#[derive(Debug)]
pub struct Message {
    pub channel: ChannelID,
    pub flags: u8,
    pub message_type: MessageType,
    pub content: Vec<u8>,
}

impl Message {
    /// Creates a new [`Message`].
    pub fn new(channel: ChannelID, flags: u8, message_type: MessageType, content: Vec<u8>) -> Message {
        Message {
            channel,
            flags,
            message_type,
            content,
        }
    }

    pub fn from_primitives(channel: u8, flags: u8, message_type: u16, content: Vec<u8>) -> Message {
        let channel_enum = ChannelID::try_from(channel).unwrap();
        let message_type_enum = MessageType::try_from(message_type).unwrap();

        Message {
            channel: channel_enum,
            flags,
            message_type: message_type_enum,
            content,
        }
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // bytes.push(self.channel);
        bytes.push(self.channel.into());
        bytes.push(self.flags);
        bytes.extend(u16::to_be_bytes((self.content.len() + 2) as u16));
        bytes.extend(self.message_type.to_be_bytes());
        bytes.extend(self.content);

        bytes
    }
}

#[derive(Debug)]
pub enum MessageTypeFlags {
    Control = 0,
    // Specific = 1 << 2
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum MessageType {
    VersionRequest = 0x1,
    VersionResponse = 0x2,
    SslHandshake = 0x3,
    AuthComplete = 0x4,
    ServiceDiscoveryRequest = 0x5,
    ServiceDiscoveryResponse = 0x6,
    ChannelOpenRequest = 0x7,
    ChannelOpenResponse = 0x8,
    PingRequest = 0xb,
    PingResponse = 0xc,
    NavigationFocusRequest = 0x0d,
    NavigationFocusResponse = 0x0e,
    VoiceSessionRequest = 0x11,
    AudioFocusRequest = 0x12,
    AudioFocusResponse = 0x13,
}

// enum MediaMessageType {
//     MediaWithTimestampIndication = 0x0000,
//     MediaIndication = 0x0001,
//     SetupRequest = 0x8000,
//     StartIndication = 0x8001,
//     SetupResponse = 0x8003,
//     MediaAckIndication = 0x8004,
//     VideoFocusIndication = 0x8008,
// }
  
// enum InputChannelMessageType {
//     None = 0,
//     Event = 0x8001,
//     HandshakeRequest = 0x8002,
//     HandshakeResponse = 0x8003,
// }

impl MessageType {
    pub fn to_be_bytes(self) -> [u8; 2] {
        u16::to_be_bytes(self as u16)
    }
}

impl TryFrom<u16> for MessageType {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == MessageType::VersionRequest as u16 => Ok(MessageType::VersionRequest),
            x if x == MessageType::VersionResponse as u16 => Ok(MessageType::VersionResponse),
            x if x == MessageType::SslHandshake as u16 => Ok(MessageType::SslHandshake),
            x if x == MessageType::AuthComplete as u16 => Ok(MessageType::AuthComplete),
            x if x == MessageType::ServiceDiscoveryRequest as u16 => Ok(MessageType::ServiceDiscoveryRequest),
            x if x == MessageType::ServiceDiscoveryResponse as u16 => Ok(MessageType::ServiceDiscoveryResponse),
            x if x == MessageType::ChannelOpenRequest as u16 => Ok(MessageType::ChannelOpenRequest),
            x if x == MessageType::ChannelOpenResponse as u16 => Ok(MessageType::ChannelOpenResponse),
            x if x == MessageType::PingRequest as u16 => Ok(MessageType::PingRequest),
            x if x == MessageType::PingResponse as u16 => Ok(MessageType::PingResponse),
            x if x == MessageType::NavigationFocusRequest as u16 => Ok(MessageType::NavigationFocusRequest),
            x if x == MessageType::NavigationFocusResponse as u16 => Ok(MessageType::NavigationFocusResponse),
            x if x == MessageType::VoiceSessionRequest as u16 => Ok(MessageType::VoiceSessionRequest),
            x if x == MessageType::AudioFocusRequest as u16 => Ok(MessageType::AudioFocusRequest),
            x if x == MessageType::AudioFocusResponse as u16 => Ok(MessageType::AudioFocusResponse),
            _ => Err(())
        }
    }
}
