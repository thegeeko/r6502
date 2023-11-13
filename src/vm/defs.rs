pub type Byte = u8;
pub type Word = u16;
pub const MEM_SIZE:usize = 1024 * 64; // 64kb
pub enum ERRORS {
    OutOfBoundary
}