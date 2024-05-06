
pub struct Canbus {
    pub baudrate: u32,
}
pub struct Current {
    pub crnt: u32,
}
pub struct Voltage {
    pub volt: u32,
}

pub enum Caliber {
    Canbus(Canbus),
    Current(Current),
    Voltage(Voltage),
}
