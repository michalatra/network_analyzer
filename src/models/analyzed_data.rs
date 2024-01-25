use chrono::DateTime;
use chrono::offset::Local;

use crate::traits::packet_analysis::PacketAnalysis;
use crate::traits::packet_filter::PacketFilter;
use crate::utils::input_util::{read_command, read_input};

pub struct AnalyzedData {
    pub timestamp: DateTime<Local>,
    pub packets: Vec<Box<dyn PacketAnalysis>>,
}

impl AnalyzedData {
    pub fn new() -> AnalyzedData {
        AnalyzedData {
            timestamp: Local::now(),
            packets: Vec::new(),
        }
    }

    pub fn apply_filters(&self, filters: &Vec<Box<dyn PacketFilter>>) -> bool {
        let mut is_valid = true;

        for filter in filters {
            let mut is_filter_valid = false;

            for packet in &self.packets {
                if filter.is_applicable(packet.protocol()) {
                    is_filter_valid = filter.apply(packet);
                    if is_filter_valid {
                        break;
                    }
                }
            }

            if !is_filter_valid {
                is_valid = false;
                break;
            }
        }

        is_valid
    }

    pub fn get_info(&self) -> String {
        let mut description = String::new();

        for packet in &self.packets {
            description.push_str(packet.short_description().as_str());
            description.push_str(";");
        }

        format!("[{}] {}", self.timestamp.format("%Y-%m-%d %H:%M:%S"), description)
    }

    pub fn print_details(&self) {
        println!("--------- Packet details ---------");
        self.print_instructions();
        println!("|");
        println!("| Timestamp: {}", self.timestamp.format("%Y-%m-%d %H:%M:%S"));
        self.print_collected_packets();
        println!("|");

        while self.interpret_command(read_command("Packet Details")) {}
    }

    fn print_collected_packets(&self) {
        println!("--------- Collected packets ---------");
        println!("|");
        for (idx, packet) in self.packets.iter().enumerate() {
            println!("| [{}]: {}", idx, packet.short_description());
        }
    }

    fn print_packet_payload(&self) {
        let packet_idx: Option<usize> = read_input("Packet index");

        let packet = match packet_idx {
            Some(idx) => self.packets.get(idx),
            None => None
        };

        match packet {
            Some(packet) => println!("| Packet payload: {:?}", packet.payload()),
            None => println!("Invalid packet index")
        }
    }

    fn print_instructions(&self) {
        println!("|");
        println!("| Type 'p' to print collected packets");
        println!("| Type 'd' to get packet payload");
        println!("| Type packet number to view packet details");
        println!("| Type 'q' to quit");
        println!("| Type 'h' to view this instructions");
        println!("|");
        println!("| Each command must be followed by pressing 'Enter'");
        println!("|");
    }

    fn interpret_command(&self, command: String) -> bool {
        match command.trim() {
            "p" => {
                self.print_collected_packets();
                true
            },
            "d" => {
                self.print_packet_payload();
                true
            },
            "q" => false,
            "h" => {
                self.print_instructions();
                true
            },
            _ => {
                self.view_packet_details(command);
                true
            }
        }
    }

    fn view_packet_details(&self, command: String) {
        let packet_idx: usize = match command.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Could not parse packet index");
                return;
            }
        };

        match self.packets.get(packet_idx) {
            Some(packet) => packet.print_details(),
            None => println!("Packet with index {} does not exist", packet_idx)
        }
    }
}