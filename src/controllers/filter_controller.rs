use crate::filters::destination_filter::DestinationFilter;
use crate::filters::protocol_filter::ProtocolFilter;
use crate::filters::source_filter::SourceFilter;
use crate::traits::packet_filter::PacketFilter;

pub struct FilterController {
    available_filters: Vec<Box<dyn PacketFilter>>,
    active_filters: Vec<Box<dyn PacketFilter>>
}

impl FilterController {
    pub fn new() -> FilterController {
        FilterController {
            available_filters: vec![
                Box::new(SourceFilter::new()),
                Box::new(DestinationFilter::new()),
                Box::new(ProtocolFilter::new()),
            ],
            active_filters: vec![]
        }
    }

    pub fn get_available_filters(&self) -> &Vec<Box<dyn PacketFilter>> {
        &self.available_filters
    }

    pub fn get_active_filters(&self) -> &Vec<Box<dyn PacketFilter>> {
        &self.active_filters
    }

    pub fn add_filter(&mut self, filter: Box<dyn PacketFilter>) {
        self.active_filters.push(filter);
    }

    pub fn remove_filter(&mut self, filter_idx: usize) {
        self.active_filters.remove(filter_idx);
    }

    pub fn configure_filter(&mut self, filter_idx: usize) {
        let filter = self.active_filters.get_mut(filter_idx).unwrap();
        filter.configure();
    }

    pub fn clear_filters(&mut self) {
        self.active_filters.clear();
    }

}