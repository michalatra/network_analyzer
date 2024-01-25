use strum_macros::{Display, EnumIter, FromRepr};

#[derive(Debug, EnumIter, Display, FromRepr, PartialEq, Eq, Hash)]
pub enum Protocol {
    ARP,
    Ethernet,
    IPv4,
    IPv6,
    TCP,
    UDP
}
