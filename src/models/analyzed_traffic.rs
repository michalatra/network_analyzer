use std::collections::HashMap;
use crate::enums::protocol::Protocol;

pub struct AnalyzedTraffic {
    protocol_usages: HashMap<Protocol, usize>,
    total_packets: usize,
    sniffing_duration: u64,
    average_packet_size: usize,
    average_packets_per_second: f64,
    max_packet_size: usize,
    min_packet_size: usize,
}

impl AnalyzedTraffic {
    pub fn new(
        protocol_usages: HashMap<Protocol, usize>,
        total_packets: usize,
        sniffing_duration: u64,
        average_packet_size: usize,
        average_packets_per_second: f64,
        max_packet_size: usize,
        min_packet_size: usize,
    ) -> AnalyzedTraffic {
        AnalyzedTraffic {
            protocol_usages,
            total_packets,
            sniffing_duration,
            average_packet_size,
            average_packets_per_second,
            max_packet_size,
            min_packet_size,
        }
    }

    pub fn get_info(&self) -> String {
        format!(
            "Total packets: {}\n\
            Sniffing duration: {} seconds\n\
            Average packet size: {} bytes\n\
            Average packets per second: {}\n\
            Max packet size: {} bytes\n\
            Min packet size: {} bytes\n\
            Protocol usages:\n\
            {}",
            self.total_packets,
            self.sniffing_duration,
            self.average_packet_size,
            self.average_packets_per_second,
            self.max_packet_size,
            self.min_packet_size,
            self.get_protocol_usages_formatted(),
        )
    }

    fn get_protocol_usages_formatted(&self) -> String {
        let mut formatted = String::new();

        for (protocol, usage) in &self.protocol_usages {
            formatted.push_str(format!("\t{}: {}\n", protocol, usage).as_str());
        }

        formatted
    }
}