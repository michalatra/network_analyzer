use std::collections::HashMap;

use pcap::Packet as PcapPacket;

use crate::enums::protocol::Protocol;
use crate::models::analyzed_data::AnalyzedData;
use crate::models::analyzed_traffic::AnalyzedTraffic;
use crate::protocols::ethernet::Ethernet;
use crate::traits::packet_analysis::PacketAnalysis;

pub struct Analyzer {}


impl Analyzer {
    pub fn analyze_packet(raw_packet: &PcapPacket) -> AnalyzedData {
        let mut packet: Option<Box<dyn PacketAnalysis>> = Ethernet::new(raw_packet.data);

        let mut analyzed_data = AnalyzedData::new();

        while packet.is_some() {
            analyzed_data.packets.push(packet.unwrap());
            packet = analyzed_data.packets.last().unwrap().next_protocol();
        }

        analyzed_data
    }

    pub fn analyze_traffic(packets: &Vec<AnalyzedData>) -> AnalyzedTraffic {
        if packets.len() == 0 {
            return AnalyzedTraffic::new(
                HashMap::new(),
                0,
                0,
                0,
                0.0,
                0,
                0,
            );
        }

        let mut protocol_usages: HashMap<Protocol, usize> = HashMap::new();
        let total_packets: usize = packets.len();
        let sniffing_duration: u64 = Analyzer::get_sniffing_duration(packets);
        let mut average_packet_size: usize = 0;
        let average_packets_per_second: f64 = total_packets as f64 / sniffing_duration as f64;
        let mut max_packet_size: usize = 0;
        let mut min_packet_size: usize = usize::MAX;

        for packet in packets {
            let packet_size = if packet.packets.len() > 0 {
                packet.packets[0].payload_length()
            } else {
                0
            };

            if packet_size > max_packet_size {
                max_packet_size = packet_size;
            }

            if packet_size < min_packet_size {
                min_packet_size = packet_size;
            }

            average_packet_size += packet_size;

            for inner_packet in &packet.packets {
                let protocol = inner_packet.protocol();
                let usage = protocol_usages.entry(protocol).or_insert(0);
                *usage += 1;
            }
        }

        average_packet_size /= total_packets;

        AnalyzedTraffic::new(
            protocol_usages,
            total_packets,
            sniffing_duration,
            average_packet_size,
            average_packets_per_second,
            max_packet_size,
            min_packet_size,
        )
    }

    fn get_sniffing_duration(packets: &Vec<AnalyzedData>) -> u64 {
        let first_packet = packets.first().unwrap();
        let last_packet = packets.last().unwrap();

        (last_packet.timestamp - first_packet.timestamp).num_seconds() as u64
    }


}