use crate::enums::protocol::Protocol;
use crate::traits::packet_analysis::PacketAnalysis;
use crate::traits::packet_filter::PacketFilter;
use crate::utils::input_util::read_command;
use crate::utils::protocol_util::select_protocol;

pub struct ProtocolFilter {
    pub(crate) accepted_protocols: Vec<Protocol>,
}

impl ProtocolFilter {
    pub fn new() -> ProtocolFilter {
        ProtocolFilter {
            accepted_protocols: Vec::new(),
        }
    }

    fn print_configurator_instructions(&self) {
        println!("------ Protocol filter configurator ------");
        println!("|");
        println!("| For the filter to be applied you must set the accepted protocols");
        println!("|");
    }

    fn print_current_config(&self) {
        println!("------ Current configuration ------");
        println!("|");
        println!("| Current accepted protocols: {:?}", self.accepted_protocols);
        println!("|");
    }

    fn print_instructions() {
        println!("------ Protocol filter ------");
        println!("|");
        println!("| Type 'a' to add a protocol");
        println!("| Type 'c' to clear all protocols");
        println!("| Type 'l' to show current protocols");
        println!("| Type 'h' to view this instructions");
        println!("| Type 'q' to quit");
        println!("|");
    }

    fn interpret_command(&mut self, command: String) -> bool {
        match command.trim() {
            "a" => {
                self.add_protocol();
                true
            },
            "c" => {
                self.clear_protocols();
                true
            },
            "l" => {
                println!("Current protocols: {:?}", self.accepted_protocols);
                true
            },
            "h" => {
                ProtocolFilter::print_instructions();
                true
            },
            "q" => false,
            _ => {
                println!("Invalid command");
                true
            }
        }
    }

    fn interpret_create_commands(command: String, protocols: &mut Vec<Protocol>) -> bool {
        match command.trim() {
            "a" => {
                ProtocolFilter::add_protocol_static(protocols);
                true
            },
            "c" => {
                ProtocolFilter::clear_protocols_static(protocols);
                true
            },
            "l" => {
                println!("Current protocols: {:?}", protocols);
                true
            },
            "h" => {
                ProtocolFilter::print_instructions();
                true
            },
            "q" => false,
            _ => {
                println!("Invalid command");
                true
            }
        }
    }

    fn add_protocol_static(protocols: &mut Vec<Protocol>) {
        let protocol = select_protocol();

        match protocol {
            Some(protocol) => protocols.push(protocol),
            None => println!("Invalid protocol"),
        }
    }

    fn add_protocol(&mut self) {
        let protocol = select_protocol();

        match protocol {
            Some(protocol) => self.accepted_protocols.push(protocol),
            None => println!("Invalid protocol"),
        }
    }

    fn clear_protocols(&mut self) {
        self.accepted_protocols.clear();
    }

    fn clear_protocols_static(protocols: &mut Vec<Protocol>) {
        protocols.clear();
    }
}

impl PacketFilter for ProtocolFilter {
    fn apply(&self, packet: &Box<dyn PacketAnalysis>) -> bool {
        self.accepted_protocols.contains(&packet.protocol())
    }

    fn get_description(&self) -> String {
        String::from("Filtering by protocol")
    }

    fn get_config(&self) -> String {
        format!("Filtering by protocols: {:?}", self.accepted_protocols)
    }

    fn configure(&mut self) {
        self.print_configurator_instructions();
        self.print_current_config();
        ProtocolFilter::print_instructions();

        while self.interpret_command(read_command("Protocol Filter")) {}
    }

    fn is_applicable(&self, protocol: Protocol) -> bool {
        true
    }

    fn create_filter(&self) -> Box<dyn PacketFilter> {
        self.print_configurator_instructions();
        ProtocolFilter::print_instructions();

        let mut protocols = Vec::new();

        while ProtocolFilter::interpret_create_commands(read_command("Protocol Filter"), &mut protocols) {}

        Box::new(ProtocolFilter {
            accepted_protocols: protocols,
        })
    }
}