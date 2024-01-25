use std::net::Ipv4Addr;

use pnet::packet::arp::{ArpHardwareType, ArpOperation, ArpPacket};
use pnet::packet::ethernet::EtherType;
use pnet::packet::Packet;
use pnet::util::MacAddr;
use crate::enums::protocol::Protocol;

use crate::traits::packet_analysis::PacketAnalysis;

pub struct ARP {
    hardware_type: ArpHardwareType,
    protocol_type: EtherType,
    hw_addr_len: u8,
    proto_addr_len: u8,
    operation: ArpOperation,
    sender_hw_addr: MacAddr,
    sender_proto_addr: Ipv4Addr,
    target_hw_addr: MacAddr,
    target_proto_addr: Ipv4Addr,
    payload: Vec<u8>

}

impl ARP {
    const PROTOCOL: Protocol = Protocol::ARP;
    pub fn new(packet_data: &[u8]) -> Option<Box<dyn PacketAnalysis>> {
        match ArpPacket::new(packet_data) {
            Some(arp) => {
                Some(Box::new(ARP {
                    hardware_type: arp.get_hardware_type(),
                    protocol_type: arp.get_protocol_type(),
                    hw_addr_len: arp.get_hw_addr_len(),
                    proto_addr_len: arp.get_proto_addr_len(),
                    operation: arp.get_operation(),
                    sender_hw_addr: arp.get_sender_hw_addr(),
                    sender_proto_addr: arp.get_sender_proto_addr(),
                    target_hw_addr: arp.get_target_hw_addr(),
                    target_proto_addr: arp.get_target_proto_addr(),
                    payload: arp.payload().to_vec()
                }))
            },
            None => None
        }
    }
}

impl PacketAnalysis for ARP {
    fn short_description(&self) -> String {
        format!("ARP {}[{}] -> {}[{}] len {}", self.sender_hw_addr, self.sender_proto_addr, self.target_hw_addr, self.target_proto_addr, self.payload.len())
    }

    fn next_protocol(&self) -> Option<Box<dyn PacketAnalysis>> {
        None
    }

    fn print_details(&self) {
        println!("-------- ARP packet --------");
        println!("|");
        println!("| Hardware type: {:?}", self.hardware_type);
        println!("| Protocol type: {:?}", self.protocol_type);
        println!("| Hardware address length: {}", self.hw_addr_len);
        println!("| Protocol address length: {}", self.proto_addr_len);
        println!("| Operation: {:?}", self.operation);
        println!("| Sender hardware address: {}", self.sender_hw_addr);
        println!("| Sender protocol address: {}", self.sender_proto_addr);
        println!("| Target hardware address: {}", self.target_hw_addr);
        println!("| Target protocol address: {}", self.target_proto_addr);
        println!("| Payload length: {}", self.payload.len());
        println!("|");
    }

    fn protocol(&self) -> Protocol {
        ARP::PROTOCOL
    }

    fn source(&self) -> String {
        format!("{}", self.sender_proto_addr)
    }

    fn destination(&self) -> String {
        format!("{}", self.target_proto_addr)
    }

    fn payload_length(&self) -> usize {
        self.payload.len()
    }

    fn payload(&self) -> &Vec<u8> {
        self.payload.as_ref()
    }
}