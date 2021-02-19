enum_list_module! {
    ConnectionType u8:
    Unknown 0,
    Ethernet 1,
    WIFI 2,
    Cell 3,
    Cell2G 4,
    Cell3G 5,
    Cell4G 6
}

impl Default for ConnectionType {
    fn default() -> Self {
        ConnectionType::Unknown
    }
}
