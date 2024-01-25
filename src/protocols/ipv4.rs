use std::net::Ipv4Addr;

use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use crate::enums::protocol::Protocol;

use crate::protocols::tcp::TCP;
use crate::protocols::udp::UDP;
use crate::traits::packet_analysis::PacketAnalysis;

pub struct IPv4 {
    source: Ipv4Addr,
    destination: Ipv4Addr,
    header_length: u8,
    dscp: u8,
    ecn: u8,
    total_length: u16,
    identification: u16,
    flags: u8,
    ttl: u8,
    checksum: u16,
    next_level_protocol: IpNextHeaderProtocol,
    payload: Vec<u8>
}

impl IPv4 {

    const PROTOCOL: Protocol = Protocol::IPv4;

    pub fn new(packet_data: &[u8]) -> Option<Box<dyn PacketAnalysis>> {
        match Ipv4Packet::new(packet_data) {
            Some(ipv4) => {
                Some(Box::new(IPv4 {
                    source: ipv4.get_source(),
                    destination: ipv4.get_destination(),
                    header_length: ipv4.get_header_length(),
                    dscp: ipv4.get_dscp(),
                    ecn: ipv4.get_ecn(),
                    total_length: ipv4.get_total_length(),
                    identification: ipv4.get_identification(),
                    flags: ipv4.get_flags(),
                    ttl: ipv4.get_ttl(),
                    checksum: ipv4.get_checksum(),
                    next_level_protocol: ipv4.get_next_level_protocol(),
                    payload: ipv4.payload().to_vec()
                }))
            },
            None => None
        }
    }
}

impl PacketAnalysis for IPv4 {
    fn short_description(&self) -> String {
        format!("IPv4 {} -> {} len {}", self.source, self.destination, self.payload.len())
    }

    fn next_protocol(&self) -> Option<Box<dyn PacketAnalysis>> {
        match self.next_level_protocol {
            IpNextHeaderProtocol(6) => TCP::new(self.payload.as_slice()),
            IpNextHeaderProtocol(17) => UDP::new(self.payload.as_slice()),
            _ => None
        }
    }

    fn print_details(&self) {
        println!("-------- IPv4 packet --------");
        println!("|");
        println!("| Source: {}", self.source);
        println!("| Destination: {}", self.destination);
        println!("| Header length: {}", self.header_length);
        println!("| DSCP: {}", self.dscp);
        println!("| ECN: {}", self.ecn);
        println!("| Total length: {}", self.total_length);
        println!("| Identification: {}", self.identification);
        println!("| Flags: {}", self.flags);
        println!("| TTL: {}", self.ttl);
        println!("| Checksum: {}", self.checksum);
        println!("| Next level protocol: {:?}", self.next_level_protocol);
        println!("| Payload length: {}", self.payload.len());
        println!("|");
    }

    fn protocol(&self) -> Protocol {
        IPv4::PROTOCOL
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