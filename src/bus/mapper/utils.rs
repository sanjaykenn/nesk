pub fn mirror_namespace_horizontal(address: u16) -> u16 {
    address & 0x3FF | if address & 0x800 != 0 { 0x400 } else { 0x000 }
}

pub fn mirror_namespace_vertical(address: u16) -> u16 {
    address & 0x7FF
}

pub fn mirror_namespace(address: u16, horizontal: bool, vertical: bool) -> u16 {
    if horizontal & vertical {
        address & 0x3FF
    } else if horizontal {
        mirror_namespace_horizontal(address)
    } else {
        mirror_namespace_vertical(address)
    }
}