
pub struct Record { // each record is 8 bytes
    time: Time, // 4 bytes
    action: Action, // 1 byte
    label_id: u16, // 2 bytes
    device: Device, // 1 byte
}

pub struct Time {
    epoch_minutes: u32,
}

pub enum Action {
    DoNotTrack,
    Stand,
    Sit,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// TODO: can labels be static??? Eg max 8 and have them be fixed in slots?
pub struct Labels {
    seq: Vec<Label>,
}

pub struct Label {
    color: Color,
    text: Vec<u8>,
}

pub enum Device {
    Unknown,
    MacOS,
    Windows,
    Linux,
    IPhone,
    IPad,
    AppleWatch,
    SmartPhone,
    Tablet,
    SmartWatch,
    Web,
    MiniComputer,
}

pub fn test() {
    println!("Test");
}