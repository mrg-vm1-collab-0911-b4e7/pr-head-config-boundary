#[repr(C)]
pub struct Access {
    pub owner: u8,
}
pub fn read_public() -> &'static str { "PUBLIC" }
