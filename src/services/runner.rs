use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::controllers::device_controller::DeviceController;
use crate::controllers::filter_controller::FilterController;
use crate::controllers::operation_controller::OperationController;
use crate::services::packet_viewer::PacketViewer;
use crate::services::sniffer::Sniffer;

pub struct Runner {
    pub device_controller: DeviceController,
    pub filter_controller: FilterController,
    pub sniffer: Sniffer,
    pub packet_viewer: PacketViewer,
    pub running: Arc<AtomicBool>,
}

impl Runner {
    pub fn new(
        running: Arc<AtomicBool>,
        sniffing_activated: Arc<AtomicBool>
    ) -> Runner {
        Runner {
            device_controller: DeviceController::new(),
            filter_controller: FilterController::new(),
            sniffer: Sniffer::new(sniffing_activated),
            packet_viewer: PacketViewer::new(),
            running,
        }
    }

    pub fn run(&mut self) {
        self.print_welcome_message();
        self.main_loop();
    }

    fn print_welcome_message(&self) {
        println!("----------------------------------");
        println!("| ");
        println!("| Welcome to the packet sniffer!");
        println!("|\n");
    }

    fn main_loop(&mut self) {
        while self.running.load(Ordering::Relaxed) {
            self.print_instructions();
            let operation = OperationController::choose_operation();

            match operation {
                Some(operation) => OperationController::perform_operation(operation, self),
                None => {
                    println!("---------- Error ----------");
                    println!("|");
                    println!("| Invalid operation");
                    println!("|\n");
                }
            }
        }
    }

    fn print_instructions(&self) {
        println!("-------- Available options -------");
        println!("| ");
        OperationController::print_operations();
        println!("|\n");
    }
}