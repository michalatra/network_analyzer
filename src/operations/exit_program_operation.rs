use std::process::exit;
use crate::traits::runner_operation::RunnerOperation;
use std::sync::atomic::Ordering;
use crate::services::runner::Runner;

pub struct ExitProgramOperation {}

impl ExitProgramOperation {
    const OPERATION: ExitProgramOperation = ExitProgramOperation {};

    pub fn new() -> ExitProgramOperation {
        ExitProgramOperation::OPERATION
    }
}

impl RunnerOperation for ExitProgramOperation {
    fn run(&self, runner: &mut Runner) {
        runner.running.store(false, Ordering::Relaxed);
        exit(0)
    }

    fn get_description(&self) -> String {
        String::from("Exit program")
    }

    fn print_instructions(&self) {
        println!("\n---------- Exit program ---------");
        println!("|");
        println!("| Exiting program...");
        println!("| Goodbye!");
        println!("|\n");
    }

    fn verify_prerequisites(&self, runner: &Runner) -> bool {
        true
    }
}