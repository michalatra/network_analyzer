use std::io::Write;
use crate::traits::runner_operation::RunnerOperation;
use crate::services::runner::Runner;

pub struct ClearCollectedPacketsOperation {}

impl ClearCollectedPacketsOperation {
    const OPERATION: ClearCollectedPacketsOperation = ClearCollectedPacketsOperation {};

    pub fn new() -> ClearCollectedPacketsOperation {
        ClearCollectedPacketsOperation::OPERATION
    }

    fn verify_agreement(&self, collected_packets_count: usize) -> bool {
        let mut input = String::new();

        println!("\n------------ Warning -----------");
        println!("|");
        println!("| Currently there are {:} packets collected.", collected_packets_count);
        println!("| Are you sure you want to clear collected packets? (y/n)");
        println!("|\n");
        print!("> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).unwrap();

        input.trim().to_lowercase() == "y"
    }
}

impl RunnerOperation for ClearCollectedPacketsOperation {
    fn run(&self, runner: &mut Runner) {
        if !self.verify_agreement(runner.sniffer.get_sniffed_packets().len()) {
            return;
        }
        runner.sniffer.clear_sniffed_packets();
    }

    fn get_description(&self) -> String {
        String::from("Clear collected packets")
    }

    fn print_instructions(&self) {
        println!("\n------- Clearing collected packets -------");
        println!("|");
        println!("| Collected packets cleared!");
        println!("|\n");
    }

    fn verify_prerequisites(&self, runner: &Runner) -> bool {
        true
    }
}