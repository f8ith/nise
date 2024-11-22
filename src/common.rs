pub mod graphics;

pub fn to_u16(l: u8, h: u8) -> u16 {
    (l as u16) | (h as u16) << 8
}
