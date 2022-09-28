
// use crate::aasdk;

// use aasdk::channel::ChannelID;
// use aasdk::messenger::encryption::EncryptionType;
// use aasdk::messenger::MessageType;

// pub struct FrameHeader {
//     channel_id: ChannelID,
//     frame_type: FrameType,
//     encryption_type: EncryptionType,
//     message_type: MessageType,
// }

pub enum FrameType {
    // Middle = 0,
    First = 1 << 0,
    Last = 1 << 1,
    Bulk = FrameType::First as isize | FrameType::Last as isize
}

// pub enum FrameSizeType {
//     Short,
//     Extended
// }

// impl FrameHeader {
//     pub fn new(channel_id: aasdk::channel::ChannelID,
//             frame_type: FrameType,
//             encryption_type: aasdk::messenger::encryption::EncryptionType,
//             message_type: aasdk::messenger::MessageType) -> FrameHeader {
//         Self { channel_id, frame_type, encryption_type, message_type }
//     }

//     pub fn get_channel_id(self) -> ChannelID {
//         self.channel_id
//     }
    
//     pub fn get_frame_type(self) -> FrameType {
//         self.frame_type
//     }

//     pub fn get_encryption_type(self) -> EncryptionType {
//         self.encryption_type
//     }

//     pub fn get_message_type(self) -> MessageType {
//         self.message_type
//     }
// }
