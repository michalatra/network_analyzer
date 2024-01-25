use crate::services::runner::Runner;
use crate::traits::runner_operation::RunnerOperation;

pub struct ViewPacketsOperation {}

impl ViewPacketsOperation {
    const OPERATION: ViewPacketsOperation = ViewPacketsOperation {};

    pub fn new() -> ViewPacketsOperation {
        ViewPacketsOperation::OPERATION
    }
}

impl RunnerOperation for ViewPacketsOperation {
    fn run(&self, runner: &mut Runner) {
        runner.packet_viewer.view_packets(
            runner.sniffer.get_sniffed_packets(),
            runner.filter_controller.get_active_filters()
        );
    }

    fn get_description(&self) -> String {
        String::from("View packets")
    }

    fn print_instructions(&self) {
        println!("\n------------ Viewing packets -----------");
    }

    fn verify_prerequisites(&self, runner: &Runner) -> bool {
        let are_packets_available = runner.sniffer.get_sniffed_packets().len() > 0;

        if !are_packets_available {
            println!("\n------------ ERROR -----------");
            println!("|");
            println!("| No packets available to view!");
            println!("| Please start sniffing first");
            println!("|\n");
        }

        are_packets_available
    }
}