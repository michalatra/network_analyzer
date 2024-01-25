use crate::enums::protocol::Protocol;

pub trait PacketAnalysis {
    fn short_description(&self) -> String;
    fn next_protocol(&self) -> Option<Box<dyn PacketAnalysis>>;
    fn print_details(&self);
    fn protocol(&self) -> Protocol;
    fn source(&self) -> String;
    fn destination(&self) -> String;
    fn payload_length(&self) -> usize;
    fn payload(&self) -> &Vec<u8>;
}