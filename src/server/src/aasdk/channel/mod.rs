
#[derive(Debug, FromPrimitive)]
pub enum ChannelID {
    Control = 0,
    Input,
    Sensor,
    Video,
    MediaAudio,
    SpeechAudio,
    SystemAudio,
    AVInput,
    Bluetooth,
    None = 255
}

impl TryFrom<u8> for ChannelID {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == ChannelID::Control as u8 => Ok(ChannelID::Control),
            x if x == ChannelID::Input as u8 => Ok(ChannelID::Input),
            x if x == ChannelID::Sensor as u8 => Ok(ChannelID::Sensor),
            x if x == ChannelID::Video as u8 => Ok(ChannelID::Video),
            x if x == ChannelID::MediaAudio as u8 => Ok(ChannelID::MediaAudio),
            x if x == ChannelID::SpeechAudio as u8 => Ok(ChannelID::SpeechAudio),
            x if x == ChannelID::SystemAudio as u8 => Ok(ChannelID::SystemAudio),
            x if x == ChannelID::AVInput as u8 => Ok(ChannelID::AVInput),
            x if x == ChannelID::Bluetooth as u8 => Ok(ChannelID::Bluetooth),
            x if x == ChannelID::None as u8 => Ok(ChannelID::None),
            _ => Err(())
        }
    }
}

impl Into<u8> for ChannelID {
    fn into(self) -> u8 {
        match self {
            ChannelID::Control => ChannelID::Control as u8,
            ChannelID::Input => ChannelID::Input as u8,
            ChannelID::Sensor => ChannelID::Sensor as u8,
            ChannelID::Video => ChannelID::Video as u8,
            ChannelID::MediaAudio => ChannelID::MediaAudio as u8,
            ChannelID::SpeechAudio => ChannelID::SpeechAudio as u8,
            ChannelID::SystemAudio => ChannelID::SystemAudio as u8,
            ChannelID::AVInput => ChannelID::AVInput as u8,
            ChannelID::Bluetooth => ChannelID::Bluetooth as u8,
            ChannelID::None => ChannelID::None as u8,
        }
    }
}

// pub enum ChannelType {
//     Video,
//     Input,
//     MaxValue
// }

// pub trait ChannelHandler {
//     fn got_channel_open_response(self) -> bool;
//     fn send_channel_open_request(self);
//     fn expect_channel_open_response(self);
// }
