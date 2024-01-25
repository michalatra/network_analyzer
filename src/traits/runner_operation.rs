use crate::services::runner::Runner;

pub trait RunnerOperation {
    fn run(&self, runner: &mut Runner);
    fn get_description(&self) -> String;
    fn print_instructions(&self);
    fn verify_prerequisites(&self, runner: &Runner) -> bool;
}