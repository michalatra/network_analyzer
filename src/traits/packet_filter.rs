use crate::enums::protocol::Protocol;
use crate::traits::packet_analysis::PacketAnalysis;

pub trait PacketFilter {
    fn apply(&self, packet: &Box<dyn PacketAnalysis>) -> bool;
    fn get_description(&self) -> String;
    fn get_config(&self) -> String;
    fn configure(&mut self);
    fn is_applicable(&self, protocol: Protocol) -> bool;
    fn create_filter(&self) -> Box<dyn PacketFilter>;
}