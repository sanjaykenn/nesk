pub fn mirror_namespace_horizontal(address: u16) -> u16 {
    let mask = if address & 0x800 == 0 { 0x2000 } else { 0x2400 };
    address & 0x3FF | mask
}

pub fn mirror_namespace_vertical(address: u16) -> u16 {
    address & ! 0x800
}

pub fn mirror_namespace(address: u16, horizontal: bool, vertical: bool) -> u16 {
    if horizontal & vertical {
        address & 0x3FF | 0x2000
    } else if horizontal {
        mirror_namespace_horizontal(address)
    } else {
        mirror_namespace_vertical(address)
    }
}