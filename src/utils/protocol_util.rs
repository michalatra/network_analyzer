use strum::IntoEnumIterator;

use crate::enums::protocol::Protocol;
use crate::utils::input_util::read_input;

pub fn select_protocol() -> Option<Protocol> {
    println!("--------- Select protocol ---------");
    println!("|");
    println!("| Available protocols:");
    print_available_protocols();
    println!("|");
    perform_protocol_selection()
}

fn print_available_protocols() {
    for (idx, protocol) in Protocol::iter().enumerate() {
        println!("| [{}] {}", idx, protocol)
    }
}

fn perform_protocol_selection() -> Option<Protocol> {
    let index: Option<usize> = read_input("Enter the index of the protocol to use");

    match index {
        Some(index) => Protocol::from_repr(index),
        None => None
    }
}