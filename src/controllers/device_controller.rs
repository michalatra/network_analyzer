use pcap::Device;

pub struct DeviceController {
    devices: Vec<Device>,
    selected_device: Option<Device>
}

impl DeviceController {
    pub fn new() -> DeviceController {
        DeviceController {
            devices: Device::list().unwrap(),
            selected_device: None
        }
    }

    pub fn get_available_devices(&self) -> Vec<Device> {
        self.devices.clone()
    }

    pub fn select_device_by_index(&mut self, index: usize) -> Option<Device> {
        if index < self.devices.len() {
            self.selected_device = Some(self.devices[index].clone());
            self.selected_device.clone()
        } else {
            None
        }
    }

    pub fn get_selected_device(&self) -> Option<&Device> {
        self.selected_device.as_ref()
    }
}