use pnet::packet::Packet;
use pnet::packet::udp::UdpPacket;
use crate::enums::protocol::Protocol;

use crate::traits::packet_analysis::PacketAnalysis;

pub struct UDP {
    source: u16,
    destination: u16,
    length: u16,
    checksum: u16,
    payload: Vec<u8>
}

impl UDP {

    const PROTOCOL: Protocol = Protocol::UDP;

    pub fn new(packet_data: &[u8]) -> Option<Box<dyn PacketAnalysis>> {
        match UdpPacket::new(packet_data) {
            Some(udp) => {
                Some(Box::new(UDP {
                    source: udp.get_source(),
                    destination: udp.get_destination(),
                    length: udp.get_length(),
                    checksum: udp.get_checksum(),
                    payload: udp.payload().to_vec()
                }))
            },
            None => None
        }
    }
}

impl PacketAnalysis for UDP {
    fn short_description(&self) -> String {
        format!("UDP: {} -> {} len {}", self.source, self.destination, self.payload.len())
    }

    fn next_protocol(&self) -> Option<Box<dyn PacketAnalysis>> {
        None
    }

    fn print_details(&self) {
        println!("--------- UDP packet ---------");
        println!("|");
        println!("| Source port: {}", self.source);
        println!("| Destination port: {}", self.destination);
        println!("| Length: {}", self.length);
        println!("| Checksum: {}", self.checksum);
        println!("| Payload length: {}", self.payload.len());
        println!("|");
    }

    fn protocol(&self) -> Protocol {
        UDP::PROTOCOL
    }

    fn source(&self) -> String {
        format!("{}", self.source)
    }

    fn destination(&self) -> String {
        format!("{}", self.destination)
    }

    fn payload_length(&self) -> usize {
        self.payload.len()
    }

    fn payload(&self) -> &Vec<u8> {
        self.payload.as_ref()
    }
}