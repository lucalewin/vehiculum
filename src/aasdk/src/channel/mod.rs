#[derive(Debug)]
pub enum Channel {
    Control = 0,
    Input,
    Sensor,
    Video,
    MediaAudio,
    SpeechAudio,
    SystemAudio,
    AvInput,
    Bluetooth,
    None = 255
}

impl TryFrom<u8> for Channel {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Channel::Control as u8 => Ok(Channel::Control),
            x if x == Channel::Input as u8 => Ok(Channel::Input),
            x if x == Channel::Sensor as u8 => Ok(Channel::Sensor),
            x if x == Channel::Video as u8 => Ok(Channel::Video),
            x if x == Channel::MediaAudio as u8 => Ok(Channel::MediaAudio),
            x if x == Channel::SpeechAudio as u8 => Ok(Channel::SpeechAudio),
            x if x == Channel::SystemAudio as u8 => Ok(Channel::SystemAudio),
            x if x == Channel::AvInput as u8 => Ok(Channel::AvInput),
            x if x == Channel::Bluetooth as u8 => Ok(Channel::Bluetooth),
            x if x == Channel::None as u8 => Ok(Channel::None),
            _ => Err(())
        }
    }
}

impl Into<u8> for Channel {
    fn into(self) -> u8 {
        match self {
            Channel::Control => Channel::Control as u8,
            Channel::Input => Channel::Input as u8,
            Channel::Sensor => Channel::Sensor as u8,
            Channel::Video => Channel::Video as u8,
            Channel::MediaAudio => Channel::MediaAudio as u8,
            Channel::SpeechAudio => Channel::SpeechAudio as u8,
            Channel::SystemAudio => Channel::SystemAudio as u8,
            Channel::AvInput => Channel::AvInput as u8,
            Channel::Bluetooth => Channel::Bluetooth as u8,
            Channel::None => Channel::None as u8,
        }
    }
}

