use crate::services::runner::Runner;
use crate::traits::runner_operation::RunnerOperation;
use crate::utils::index_util::select_index;
use crate::utils::input_util::read_command;

pub struct ConfigureFiltersOperation {}

impl ConfigureFiltersOperation {
    const OPERATION: ConfigureFiltersOperation = ConfigureFiltersOperation {};

    pub fn new() -> ConfigureFiltersOperation {
        ConfigureFiltersOperation::OPERATION
    }

    fn print_current_filters(&self, runner: &Runner) {
        let filters = runner.filter_controller.get_active_filters();

        println!("------ Currently Active filters ------");
        println!("|");

        for (idx, filter) in filters.iter().enumerate() {
            println!("| [{}] {}", idx, filter.get_config());
        }

        println!("|");
    }

    fn print_available_filters(&self, runner: &Runner) {
        let filters = runner.filter_controller.get_available_filters();

        println!("------ Available filters ------");
        println!("|");

        for (idx, filter) in filters.iter().enumerate() {
            println!("| [{}] {}", idx, filter.get_description());
        }

        println!("|");
    }

    fn add_filter(&self, runner: &mut Runner) {
        println!("------ Add filter ------");
        println!("|");
        println!("| To add a filter, select one of the available filters below");
        println!("|");
        self.print_available_filters(runner);

        let filter_index = select_index(
            "Enter the index of the filter to add",
            runner.filter_controller.get_available_filters().len()
        );

        match filter_index {
            Some(filter_index) => {
                let filter = runner.filter_controller.get_available_filters()
                    .get(filter_index)
                    .unwrap()
                    .create_filter();

                runner.filter_controller.add_filter(filter);
            },
            None => println!("Invalid filter index")
        }
    }

    fn configure_filter(&self, runner: &mut Runner) {
        println!("------ Configure filter ------");
        println!("|");
        println!("| Select the filter you want to configure");
        println!("|");
        self.print_current_filters(runner);

        let filter_index = select_index(
            "Enter the index of the filter to configure",
            runner.filter_controller.get_active_filters().len()
        );

        match filter_index {
            Some(filter_index) => runner.filter_controller.configure_filter(filter_index),
            None => println!("Invalid filter index")
        }
    }

    fn remove_filter(&self, runner: &mut Runner) {
        println!("------ Remove filter ------");
        println!("|");
        println!("| Select the filter you want to remove");
        println!("|");
        self.print_current_filters(runner);

        let filter_index = select_index(
            "Enter the index of the filter to remove",
            runner.filter_controller.get_active_filters().len()
        );

        match filter_index {
            Some(filter_index) => runner.filter_controller.remove_filter(filter_index),
            None => println!("Invalid filter index")
        }
    }

    fn print_filter_options(&self) {
        println!("------ Filter options ------");
        println!("|");
        println!("| Type 'l' to list all active filters");
        println!("| Type 'a' to add a new filter");
        println!("| Type 'c' to configure an existing filter");
        println!("| Type 'r' to remove an existing filter");
        println!("| Type 'e' to clear all filters");
        println!("| Type 'q' to quit");
        println!("| Type 'h' to view this instructions");
        println!("|");
        println!("| Each command must be followed by pressing 'Enter'");
        println!("|");
    }

    fn interpret_command(&self, command: String, runner: &mut Runner) -> bool {
        match command.trim() {
            "l" => {
                self.print_current_filters(runner);
                true
            },
            "a" => {
                self.add_filter(runner);
                true
            },
            "c" => {
                self.configure_filter(runner);
                true
            },
            "r" => {
                self.remove_filter(runner);
                true
            },
            "e" => {
                runner.filter_controller.clear_filters();
                true
            },
            "q" => false,
            "h" => {
                self.print_filter_options();
                true
            },
            _ => {
                println!("Invalid command");
                true
            }
        }
    }

}

impl RunnerOperation for ConfigureFiltersOperation {
    fn run(&self, runner: &mut Runner) {
        while self.interpret_command(read_command("Filters Configurator"), runner) {}
    }

    fn get_description(&self) -> String {
        String::from("Configure filters")
    }

    fn print_instructions(&self) {
        println!("------- Filters Configurator -------");
        println!("|");
        println!("| Configure filters to filter out packets");
        println!("| based on their properties");
        println!("|");
        self.print_filter_options();
    }

    fn verify_prerequisites(&self, runner: &Runner) -> bool {
        true
    }
}