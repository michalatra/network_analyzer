use pcap::Device;
use crate::controllers::device_controller::DeviceController;
use crate::services::runner::Runner;

use crate::traits::runner_operation::RunnerOperation;
use crate::utils::input_util::read_input;

pub struct SelectDeviceOperation {}

impl SelectDeviceOperation {
    const OPERATION: SelectDeviceOperation = SelectDeviceOperation {};

    pub fn new() -> SelectDeviceOperation {
        SelectDeviceOperation::OPERATION
    }

    fn print_available_devices(&self, devices: Vec<Device>) {
        for (index, device) in devices.iter().enumerate() {
            println!("{}: {:}", index, self.get_device_representation(&device));
        }
    }

    fn get_device_representation(&self, device: &Device) -> String {
        format!("{:}", device.clone().desc.unwrap_or(device.name.to_string()))
    }

    fn perform_device_selection(&self, device_controller: &mut DeviceController) -> Option<Device> {
        let index = read_input("Enter the index of the device to use");

        match index {
            Some(index) => device_controller.select_device_by_index(index),
            None => None
        }
    }
}

impl RunnerOperation for SelectDeviceOperation {
    fn run(&self, runner: &mut Runner) {
        let devices = runner.device_controller.get_available_devices();

        self.print_available_devices(devices);
        self.perform_device_selection(&mut runner.device_controller);
    }

    fn get_description(&self) -> String {
        String::from("Select device")
    }

    fn print_instructions(&self) {
        println!("\n---------------- Select Device -------------------");
        println!("|");
        println!("| Select the device you want to sniff packets from.");
        println!("| You can select a device by entering its index.");
        println!("|\n");
    }

    fn verify_prerequisites(&self, runner: &Runner) -> bool {
        true
    }
}