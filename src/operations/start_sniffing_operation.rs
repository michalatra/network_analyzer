use crate::services::runner::Runner;
use crate::traits::runner_operation::RunnerOperation;

pub struct StartSniffingOperation {}

impl StartSniffingOperation {
    const OPERATION: StartSniffingOperation = StartSniffingOperation {};

    pub fn new() -> StartSniffingOperation {
        StartSniffingOperation::OPERATION
    }
}

impl RunnerOperation for StartSniffingOperation {
    fn run(&self, runner: &mut Runner) {
        match runner.device_controller.get_selected_device() {
            Some(device) => runner.sniffer.sniff(device),
            None => println!("No device selected, cannot start sniffing!")
        }
    }

    fn get_description(&self) -> String {
        String::from("Start sniffing")
    }

    fn print_instructions(&self) {
        println!("\n------------ Sniffing -----------");
        println!("|");
        println!("| Press Ctrl+C to stop sniffing");
        println!("|\n");
    }

    fn verify_prerequisites(&self, runner: &Runner) -> bool {
        let is_device_selected = runner.device_controller.get_selected_device().is_some();

        if !is_device_selected {
            println!("\n------------ ERROR -----------");
            println!("|");
            println!("| Sniffing device is not selected, cannot start sniffing!");
            println!("| Please select a sniffing device first");
            println!("|\n");
        }

        is_device_selected
    }
}