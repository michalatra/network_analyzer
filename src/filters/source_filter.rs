use std::io::Write;

use crate::enums::protocol::Protocol;
use crate::traits::packet_analysis::PacketAnalysis;
use crate::traits::packet_filter::PacketFilter;
use crate::utils::protocol_util::select_protocol;

pub struct SourceFilter {
    accepted_source: String,
    applicable_protocol: Protocol,
}

impl SourceFilter {
    pub fn new() -> SourceFilter {
        SourceFilter {
            accepted_source: String::from(""),
            applicable_protocol: Protocol::IPv4,
        }
    }

    fn print_configurator_instructions(&self) {
        println!("------ Source filter configurator ------");
        println!("|");
        println!("| For the filter to be applied you must set the source address");
        println!("| and protocol to which the filter will be applied.");
        println!("|");
    }

    fn print_current_config(&self) {
        println!("------ Current configuration ------");
        println!("|");
        println!("| Current source address: {}", self.accepted_source);
        println!("| Current applicable protocol: {:?}", self.applicable_protocol);
        println!("|");
    }

    fn collect_source_address(&self) -> Option<String> {
        let mut source_address = String::new();
        print!("[Source Filter] Enter source address > ");
        std::io::stdout().flush().unwrap();

        match std::io::stdin().read_line(&mut source_address) {
            Ok(_) => Some(source_address.trim().to_string()),
            Err(_) => None
        }
    }

    fn collect_protocol(&self) -> Option<Protocol> {
        select_protocol()
    }
}

impl PacketFilter for SourceFilter {
    fn apply(&self, packet: &Box<dyn PacketAnalysis>) -> bool {
        packet.source() == self.accepted_source
    }

    fn get_description(&self) -> String {
        String::from("Filtering by source address")
    }

    fn get_config(&self) -> String {
        format!("Filtering by source address: {} [{}]", self.accepted_source, self.applicable_protocol)
    }

    fn configure(&mut self) {
        self.print_configurator_instructions();
        self.print_current_config();

        let source_address = self.collect_source_address();
        let protocol = self.collect_protocol();

        match source_address {
            Some(source_address) => self.accepted_source = source_address,
            None => println!("| Given source address is invalid, sticking to the previous one {}", self.accepted_source)

        }

        match protocol {
            Some(protocol) => self.applicable_protocol = protocol,
            None => println!("| Given protocol is invalid, sticking to the previous one {:?}", self.applicable_protocol)
        }

    }

    fn is_applicable(&self, protocol: Protocol) -> bool {
        protocol == self.applicable_protocol
    }

    fn create_filter(&self) -> Box<dyn PacketFilter> {
        self.print_configurator_instructions();

        let mut source_address = self.collect_source_address();

        while source_address.is_none() {
            println!("| Given source address is invalid, try again");
            source_address = self.collect_source_address();
        }

        let mut protocol = self.collect_protocol();

        while protocol.is_none() {
            println!("| Given protocol is invalid, try again");
            protocol = self.collect_protocol();
        }

        Box::new(SourceFilter {
            accepted_source: source_address.unwrap(),
            applicable_protocol: protocol.unwrap(),
        })
    }
}