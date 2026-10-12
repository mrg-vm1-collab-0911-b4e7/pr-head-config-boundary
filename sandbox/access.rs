#[repr(C)]
pub struct Access {
    pub owner: u8,
    pub audit: u32,
}
pub fn read_public() -> &'static str { "PUBLIC" }
pub fn audit_marker() -> u8 { 1 }
pub fn metrics_marker() -> u8 { 2 }
