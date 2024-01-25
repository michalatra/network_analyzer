use crate::services::runner::Runner;
use crate::traits::runner_operation::RunnerOperation;

pub struct TrafficAnalysisOperation {}

impl TrafficAnalysisOperation {
    const OPERATION: TrafficAnalysisOperation = TrafficAnalysisOperation {};

    pub fn new() -> TrafficAnalysisOperation {
        TrafficAnalysisOperation::OPERATION
    }
}

impl RunnerOperation for TrafficAnalysisOperation {
    fn run(&self, runner: &mut Runner) {
        let traffic_analysis = runner.sniffer.get_traffic_analysis();
        println!("{}", traffic_analysis.get_info());
        println!("|");
    }

    fn get_description(&self) -> String {
        String::from("Traffic analysis")
    }

    fn print_instructions(&self) {
        println!("------------ Traffic Analysis -----------");
        println!("|");
    }

    fn verify_prerequisites(&self, runner: &Runner) -> bool {
        let has_sniffing_data = runner.sniffer.get_sniffed_packets().len() > 0;

        if !has_sniffing_data {
            println!("\n------------ ERROR -----------");
            println!("|");
            println!("| No sniffing data available, cannot perform traffic analysis!");
            println!("| Please start sniffing first");
            println!("|\n");
        }

        has_sniffing_data
    }
}