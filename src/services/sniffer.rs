use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use pcap::{Active, Capture, Device, Error, Packet};

use crate::models::analyzed_data::AnalyzedData;
use crate::models::analyzed_traffic::AnalyzedTraffic;
use crate::services::analyzer::Analyzer;

pub struct Sniffer {
    sniffed_packets: Vec<AnalyzedData>,
    sniffing_activated: Arc<AtomicBool>
}

impl Sniffer {
    pub fn new(sniffing_activated: Arc<AtomicBool>) -> Sniffer {
        Sniffer {
            sniffed_packets: Vec::new(),
            sniffing_activated
        }
    }

    pub fn sniff(& mut self, sniffing_device: &Device) {
        println!("Initializing traffic analysis...");

        self.sniffing_activated.store(true, Ordering::Relaxed);
        self.init_sniffing(&sniffing_device);

        println!("Traffic analysis finished");
    }

    pub fn get_sniffed_packets(&self) -> &Vec<AnalyzedData> {
        &self.sniffed_packets
    }

    pub fn clear_sniffed_packets(&mut self) {
        self.sniffed_packets.clear();
    }

    pub fn get_traffic_analysis(&self) -> AnalyzedTraffic {
        Analyzer::analyze_traffic(&self.sniffed_packets)
    }

    fn init_sniffing(& mut self, sniffing_device: &Device) {
        let capture = self.get_capture(&sniffing_device);

        match capture {
            Ok(cap) => self.sniff_loop(cap),
            Err(error) => println!("Error while creating capture: {}", error)
        }
    }

    fn sniff_loop(& mut self, mut capture: Capture<Active>) {
        while self.sniffing_activated.load(Ordering::Relaxed) {
            match capture.next_packet() {
                Ok(packet) => self.handle_sniffed_packet(&packet),
                Err(error) => println!("Error while reading packet: {}", error)
            }
        }
    }

    fn get_capture(&self, sniffing_device: &Device) -> Result<Capture<Active>, Error> {
        let capture = Capture::from_device(sniffing_device.clone());

        match capture {
            Ok(capture) => capture
                .promisc(true)
                .snaplen(5000)
                .timeout(1000)
                .open(),
            Err(error) => Err(error)
        }
    }

    fn handle_sniffed_packet(&mut self, packet: &Packet) {
        let anayzed_data = Analyzer::analyze_packet(packet);
        println!("{}", anayzed_data.get_info());

        self.sniffed_packets.push(anayzed_data);
    }
}