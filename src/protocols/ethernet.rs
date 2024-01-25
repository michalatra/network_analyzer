use pnet::packet::ethernet::{EthernetPacket, EtherType};
use pnet::packet::Packet;
use pnet::util::MacAddr;
use crate::enums::protocol::Protocol;

use crate::protocols::arp::ARP;
use crate::protocols::ipv4::IPv4;
use crate::protocols::ipv6::IPv6;
use crate::traits::packet_analysis::PacketAnalysis;

pub struct Ethernet {
    destination: MacAddr,
    source: MacAddr,
    ethertype: EtherType,
    payload: Vec<u8>
}


impl Ethernet {

    const PROTOCOL: Protocol = Protocol::Ethernet;

    pub fn new(packet_data: &[u8]) -> Option<Box<dyn PacketAnalysis>> {
        match EthernetPacket::new(packet_data) {
            Some(ethernet) => {
                Some(Box::new(Ethernet {
                    destination: ethernet.get_destination(),
                    source: ethernet.get_source(),
                    ethertype: ethernet.get_ethertype(),
                    payload: ethernet.payload().to_vec()
                }))
            },
            None => None
        }
    }
}

impl PacketAnalysis for Ethernet {
    fn short_description(&self) -> String {
        format!("ETHERNET: {} -> {} len {}", self.source, self.destination, self.payload.len())
    }

    fn next_protocol(&self) -> Option<Box<dyn PacketAnalysis>> {
        match self.ethertype {
            EtherType(0x0800) => IPv4::new(self.payload.as_slice()),
            EtherType(0x86DD) => IPv6::new(self.payload.as_slice()),
            EtherType(0x0806) => ARP::new(self.payload.as_slice()),
            _ => None
        }
    }

    fn print_details(&self) {
        println!("--------- Ethernet packet ---------");
        println!("|");
        println!("| Destination: {}", self.destination);
        println!("| Source: {}", self.source);
        println!("| EtherType: {:?}", self.ethertype);
        println!("| Payload length: {}", self.payload.len());
        println!("|");
    }

    fn protocol(&self) -> Protocol {
        Ethernet::PROTOCOL
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