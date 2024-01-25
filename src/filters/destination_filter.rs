use std::io::Write;

use crate::enums::protocol::Protocol;
use crate::traits::packet_analysis::PacketAnalysis;
use crate::traits::packet_filter::PacketFilter;
use crate::utils::protocol_util::select_protocol;

pub struct DestinationFilter {
    accepted_destination: String,
    applicable_protocol: Protocol,
}

impl DestinationFilter {
    pub fn new() -> DestinationFilter {
        DestinationFilter {
            accepted_destination: String::from(""),
            applicable_protocol: Protocol::IPv4,
        }
    }

    fn print_configurator_instructions(&self) {
        println!("------ Destination filter configurator ------");
        println!("|");
        println!("| For the filter to be applied you must set the destination address");
        println!("| and protocol to which the filter will be applied.");
        println!("|");
    }

    fn print_current_config(&self) {
        println!("------ Current configuration ------");
        println!("|");
        println!("| Current destination address: {}", self.accepted_destination);
        println!("| Current applicable protocol: {:?}", self.applicable_protocol);
        println!("|");
    }

    fn collect_destination_address(&self) -> Option<String> {
        let mut destination_address = String::new();
        print!("[Destination Filter] Enter destination address > ");
        std::io::stdout().flush().unwrap();

        match std::io::stdin().read_line(&mut destination_address) {
            Ok(_) => Some(destination_address.trim().to_string()),
            Err(_) => None
        }
    }

    fn collect_protocol(&self) -> Option<Protocol> {
        select_protocol()
    }
}

impl PacketFilter for DestinationFilter {
    fn apply(&self, packet: &Box<dyn PacketAnalysis>) -> bool {
        packet.destination() == self.accepted_destination
    }

    fn get_description(&self) -> String {
        String::from("Filtering by destination address")
    }

    fn get_config(&self) -> String {
        format!("Filtering by destination address: {} [{}]", self.accepted_destination, self.applicable_protocol)
    }

    fn configure(&mut self) {
        self.print_configurator_instructions();
        self.print_current_config();

        let destination_address = self.collect_destination_address();
        let protocol = self.collect_protocol();

        match destination_address {
            Some(destination_address) => self.accepted_destination = destination_address,
            None => println!("| Given destination address is invalid, sticking to the previous one {}", self.accepted_destination)

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

        let mut destination_address = self.collect_destination_address();

        while destination_address.is_none() {
            println!("| Given destination address is invalid, try again");
            destination_address = self.collect_destination_address();
        }

        let mut protocol = self.collect_protocol();

        while protocol.is_none() {
            println!("| Given protocol is invalid, try again");
            protocol = self.collect_protocol();
        }

        Box::new(DestinationFilter {
            accepted_destination: destination_address.unwrap(),
            applicable_protocol: protocol.unwrap(),
        })
    }
}