use std::cmp::min;

use crate::models::analyzed_data::AnalyzedData;
use crate::traits::packet_filter::PacketFilter;
use crate::utils::input_util::{read_command, read_input};

pub struct PacketViewer {
    window_size: usize,
    current_idx: usize,
}

impl PacketViewer {
    pub fn new() -> PacketViewer {
        PacketViewer {
            window_size: 10,
            current_idx: 0,
        }
    }

    pub fn view_packets(&mut self, packets: &Vec<AnalyzedData>, filters: &Vec<Box<dyn PacketFilter>>) {
        self.current_idx = 0;
        self.print_instructions();

        let filtered_packets = PacketViewer::get_filtered_packets(packets, filters);

        self.print_current_window(&filtered_packets, filters);

        while self.current_idx != usize::MAX {
            let command = read_command("Packet Viewer");
            self.interpret_command(command, &filtered_packets, filters);
        }
    }

    fn get_filtered_packets<'a>(packets: &'a Vec<AnalyzedData>, filters: &'a Vec<Box<dyn PacketFilter>>) -> Vec<&'a AnalyzedData> {
        let mut filtered_packets = Vec::new();

        for packet in packets {
            if packet.apply_filters(filters) {
                filtered_packets.push(packet);
            }
        }

        filtered_packets
    }

    fn print_instructions(&self) {
        println!("------------ Packet Viewer ------------");
        println!("|");
        println!("| Type 'n' to view next window");
        println!("| Type 'p' to view previous window");
        println!("| Type 'c' to view current window");
        println!("| Type 'q' to quit");
        println!("| Type 's' to change window size");
        println!("| Type packet number to view packet details");
        println!("| Type 'h' to view this instructions");
        println!("|");
        println!("| Each command must be followed by pressing 'Enter'");
        println!("|\n");
    }

    fn interpret_command(&mut self, command: String, packets: &Vec<&AnalyzedData>, filters: &Vec<Box<dyn PacketFilter>>) {
        match command.trim() {
            "n" => self.switch_to_next_window(packets, filters),
            "p" => self.switch_to_previous_window(packets, filters),
            "c" => self.print_current_window(packets, filters),
            "q" => self.current_idx = usize::MAX,
            "s" => self.change_window_size(),
            "h" => self.print_instructions(),
            _ => self.view_packet_details(command, packets)
        }
    }

    fn print_current_window(&self, packets: &Vec<&AnalyzedData>, filters: &Vec<Box<dyn PacketFilter>>) {
        let window = self.get_current_window(packets);

        match window {
            Some(window) => self.print_window(window, packets.len(), filters),
            None => self.print_info()
        }
    }

    fn print_window(&self, window: &[&AnalyzedData], packets_count: usize, filters: &Vec<Box<dyn PacketFilter>>) {
        for (idx, packet) in window.iter().enumerate() {
            if packet.apply_filters(filters) {
                println!("[{}]: {}", idx + self.current_idx + 1, packet.get_info());
            }
        }

        println!("{} - {} / {}", self.current_idx + 1, self.current_idx + window.len(), packets_count);
    }

    fn print_info(&self) {
        println!("\n------------- Info ---------------");
        println!("|");
        println!("| There are no more packets to show");
        println!("|\n");
    }

    fn get_current_window<'a>(&'a self, packets: &'a Vec<&AnalyzedData>) -> Option<&[&AnalyzedData]> {
        if packets.len() < 1 {
            return None;
        }

        let start_idx = min(self.current_idx, packets.len() - 1);
        let end_idx = min(start_idx + self.window_size, packets.len());

        if end_idx - start_idx < 1 {
            return None;
        }

        Some(&packets[start_idx..end_idx])
    }

    fn change_window_size(&mut self) {
        let new_window_size: Option<usize> = read_input("Enter new window size");

        match new_window_size {
            Some(new_window_size) => if new_window_size > 0 { self.window_size = new_window_size} else { println!("Window size must be greater than 0") },
            None => println!("Could not parse window size")
        }
    }

    fn switch_to_next_window(&mut self, packets: &Vec<&AnalyzedData>, filters: &Vec<Box<dyn PacketFilter>>) {
        self.current_idx = if self.current_idx + self.window_size < packets.len() { self.current_idx + self.window_size } else { self.current_idx };
        self.print_current_window(packets, filters);
    }

    fn switch_to_previous_window(&mut self, packets: &Vec<&AnalyzedData>, filters: &Vec<Box<dyn PacketFilter>>) {
        self.current_idx = if self.current_idx > self.window_size { self.current_idx - self.window_size } else { 0 };
        self.print_current_window(packets, filters);
    }

    fn view_packet_details(&self, command: String, packets: &Vec<&AnalyzedData>) {
        let packet_idx: usize = match command.trim().parse() {
            Ok(packet_idx) => packet_idx,
            Err(_) => {
                println!("Could not parse packet index");
                return;
            }
        };

        if packet_idx < 1 || packet_idx > packets.len() {
            println!("Packet with index {} does not exist", packet_idx);
            return;
        }

        match packets.get(packet_idx - 1) {
            Some(packet) => packet.print_details(),
            None => println!("Packet with index {} does not exist", packet_idx)
        }
    }

}