
pub enum PacketType {
    GetChannelNumberByChannelType,
    RawData,
    GetServiceDescriptor,
} 

pub struct Packet {
    packet_type: PacketType,
    channel_number: u8,
    specific: u8,
    data: Vec<u8>,
}
