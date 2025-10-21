use crate::bus::mapper::mapper_00::Mapper00;
use crate::bus::mapper::Mapper;

pub fn from_ines(binary: &[u8]) -> Result<Box<dyn Mapper>, String> {
    if binary.len() < 16 {
        return Err("Invalid binary size".to_string())
    }

    let header = &binary[0..16];

    if header[0..4] != [0x4e, 0x45, 0x53, 0x1a] {
        return Err("Invalid header".to_string())
    }

    let mut offset = 16;

    if header[6] & 0b0000_0100 != 0 {
        offset += 512;
    }

    if binary.len() < offset {
        return Err("Invalid binary size".to_string())
    }

    let prg_size = header[4] as usize * 0x4000;
    let chr_size = header[5] as usize * 0x2000;
    let chr_ram_size = if header[5] == 0 { 0x2000 } else { 0 };

    if binary.len() < offset + prg_size + chr_size {
        return Err("Invalid binary size".to_string())
    }

    let prg = binary[offset..(offset + prg_size)].to_vec();
    offset += prg_size;

    let chr = binary[offset..(offset + chr_size)].to_vec();

    let horizontal_mirror = header[6] & 1 == 0;

    Ok(Box::new(Mapper00::new(horizontal_mirror, prg.into_boxed_slice(), chr.into_boxed_slice(), 0, chr_ram_size)?))
}