use pnet::packet::Packet;
use pnet::packet::tcp::TcpPacket;
use crate::enums::protocol::Protocol;

use crate::traits::packet_analysis::PacketAnalysis;

pub struct TCP {
    source: u16,
    destination: u16,
    payload_length: usize,
    sequence: u32,
    acknowledgement: u32,
    data_offset: u8,
    reserved: u8,
    flags: u8,
    window: u16,
    checksum: u16,
    payload: Vec<u8>
}

impl TCP {

    const PROTOCOL: Protocol = Protocol::TCP;

    pub fn new(packet_data: &[u8]) -> Option<Box<dyn PacketAnalysis>> {
        match TcpPacket::new(packet_data) {
            Some(tcp) => {
                Some(Box::new(TCP {
                    source: tcp.get_source(),
                    destination: tcp.get_destination(),
                    payload_length: tcp.payload().len(),
                    sequence: tcp.get_sequence(),
                    acknowledgement: tcp.get_acknowledgement(),
                    data_offset: tcp.get_data_offset(),
                    reserved: tcp.get_reserved(),
                    flags: tcp.get_flags(),
                    window: tcp.get_window(),
                    checksum: tcp.get_checksum(),
                    payload: tcp.payload().to_vec()
                }))
            },
            None => None
        }
    }
}

impl PacketAnalysis for TCP {
    fn short_description(&self) -> String {
        format!("TCP: {} -> {} len {}", self.source, self.destination, self.payload.len())
    }

    fn next_protocol(&self) -> Option<Box<dyn PacketAnalysis>> {
        None
    }

    fn print_details(&self) {
        println!("--------- TCP packet ---------");
        println!("|");
        println!("| Source port: {}", self.source);
        println!("| Destination port: {}", self.destination);
        println!("| Payload length: {}", self.payload_length);
        println!("| Sequence: {}", self.sequence);
        println!("| Acknowledgement: {}", self.acknowledgement);
        println!("| Data offset: {}", self.data_offset);
        println!("| Reserved: {}", self.reserved);
        println!("| Flags: {}", self.flags);
        println!("| Window: {}", self.window);
        println!("| Checksum: {}", self.checksum);
        println!("|");
    }

    fn protocol(&self) -> Protocol {
        TCP::PROTOCOL
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