use std::net::Ipv6Addr;

use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;
use crate::enums::protocol::Protocol;

use crate::protocols::tcp::TCP;
use crate::protocols::udp::UDP;
use crate::traits::packet_analysis::PacketAnalysis;

pub struct IPv6 {
    source: Ipv6Addr,
    destination: Ipv6Addr,
    traffic_class: u8,
    flow_label: u32,
    payload_length: u16,
    next_header: IpNextHeaderProtocol,
    hop_limit: u8,
    payload: Vec<u8>
}

impl IPv6 {

    const PROTOCOL: Protocol = Protocol::IPv6;

    pub fn new(packet_data: &[u8]) -> Option<Box<dyn PacketAnalysis>> {
        match Ipv6Packet::new(packet_data) {
            Some(ipv6) => {
                Some(Box::new(IPv6 {
                    source: ipv6.get_source(),
                    destination: ipv6.get_destination(),
                    traffic_class: ipv6.get_traffic_class(),
                    flow_label: ipv6.get_flow_label(),
                    payload_length: ipv6.get_payload_length(),
                    next_header: ipv6.get_next_header(),
                    hop_limit: ipv6.get_hop_limit(),
                    payload: ipv6.payload().to_vec()
                }))
            },
            None => None
        }
    }
}

impl PacketAnalysis for IPv6 {
    fn short_description(&self) -> String {
        format!("IPv6 {} -> {} len {}", self.source, self.destination, self.payload.len())
    }

    fn next_protocol(&self) -> Option<Box<dyn PacketAnalysis>> {
        match self.next_header {
            IpNextHeaderProtocol(6) => TCP::new(self.payload.as_slice()),
            IpNextHeaderProtocol(17) => UDP::new(self.payload.as_slice()),
            _ => None
        }
    }

    fn print_details(&self) {
        println!("--------- IPv6 packet ---------");
        println!("|");
        println!("| Source: {}", self.source);
        println!("| Destination: {}", self.destination);
        println!("| Traffic class: {}", self.traffic_class);
        println!("| Flow label: {}", self.flow_label);
        println!("| Payload length: {}", self.payload_length);
        println!("| Next header: {:?}", self.next_header);
        println!("| Hop limit: {}", self.hop_limit);
        println!("|");
    }

    fn protocol(&self) -> Protocol {
        IPv6::PROTOCOL
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
