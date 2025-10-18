mod controller;

pub struct Controller {
    value: u8,
    strobe: u8,
    buttons: [bool; 8],
}