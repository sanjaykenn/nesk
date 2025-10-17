use std::fs::File;
use std::io::Read;
use crate::bus::mapper::Mapper;
use crate::bus::mapper::mapper_00::Mapper00;

pub fn from_ines(path: &str) -> Result<Box<dyn Mapper>, String> {
    let mut file = File::open(path).unwrap();
    let mut header = [0u8; 16];
    file.read(&mut header).unwrap();

    if header[0 .. 4] != [0x4e, 0x45, 0x53, 0x1a] {
        return Err("Invalid header".to_string())
    }

    if header[6] & 0b0000_0100 != 0 {
        let mut trainer = [0u8; 512];
        file.read(&mut trainer).unwrap();
    }

    let mut prg = vec![0u8; header[4] as usize * 0x4000];
    file.read(&mut prg).unwrap();

    let mut chr = vec![0u8; if header[5] == 0 { 1 } else { header[5] } as usize * 0x2000];
    file.read(&mut chr).unwrap();

    let horizontal_mirror = header[6] & 1 == 0;

    Ok(Box::new(Mapper00::new(horizontal_mirror, prg.into_boxed_slice(), chr.into_boxed_slice(), 0)?))
}