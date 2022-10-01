#[derive(Debug)]
#[derive(PartialEq, Eq, Clone, Copy)]
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
    ShutdownRequest = 0x000f,
    ShutdownResponse = 0x0010,
    VoiceSessionRequest = 0x11,
    AudioFocusRequest = 0x12,
    AudioFocusResponse = 0x13,
}

impl Into<u16> for MessageType {
    fn into(self) -> u16 {
        self as u16
    }
}

impl TryFrom<[u8; 2]> for MessageType {
    type Error = ();

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        let value = u16::from_be_bytes(value);

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


#[derive(Debug)]
pub struct Message {
    message_type: MessageType,
    payload: Vec<u8>,
}

impl Message {
    pub fn new(message_type: MessageType, payload: Vec<u8>) -> Self {
        Self { message_type, payload }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend(u16::to_be_bytes((self.payload.len() + 2) as u16));
        bytes.extend(u16::to_be_bytes(self.message_type.into()));
        bytes.extend(&self.payload);

        bytes
    }

    pub fn message_type(&self) -> MessageType {
        self.message_type
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}
