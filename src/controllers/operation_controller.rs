use crate::operations::clear_collected_packets_operation::ClearCollectedPacketsOperation;
use crate::operations::configure_filters_operation::ConfigureFiltersOperation;
use crate::operations::exit_program_operation::ExitProgramOperation;
use crate::operations::select_device_operation::SelectDeviceOperation;
use crate::operations::start_sniffing_operation::StartSniffingOperation;
use crate::operations::traffic_analysis_operation::TrafficAnalysisOperation;
use crate::operations::view_packets_operation::ViewPacketsOperation;
use crate::services::runner::Runner;
use crate::traits::runner_operation::RunnerOperation;
use crate::utils::input_util::read_input;

pub struct OperationController {}

impl OperationController {
    pub fn get_operations() -> Vec<Box<dyn RunnerOperation>> {
        vec![
            Box::new(SelectDeviceOperation::new()),
            Box::new(StartSniffingOperation::new()),
            Box::new(ClearCollectedPacketsOperation::new()),
            Box::new(ViewPacketsOperation::new()),
            Box::new(ConfigureFiltersOperation::new()),
            Box::new(TrafficAnalysisOperation::new()),
            Box::new(ExitProgramOperation::new()),
        ]
    }


    pub fn print_operations() {
        for (index, operation) in OperationController::get_operations().iter().enumerate() {
            println!("| [{}]: {}", index, operation.get_description());
        }
    }

    pub fn choose_operation() -> Option<usize> {
        let index: Option<usize> = read_input("Enter the index of the operation to perform");

        match index {
            Some(index) => if OperationController::get_operations().len() > index { Some(index) } else { None },
            None => None
        }
    }

    pub fn perform_operation(operation_idx: usize, runner: &mut Runner) {
        let binding = OperationController::get_operations();
        let operation = binding.get(operation_idx).unwrap();

        if operation.verify_prerequisites(runner) {
            operation.print_instructions();
            operation.run(runner);
        }
    }
}