#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlockID {
    Air,
    Stone,
}

pub const CHUNKSIZE: usize = 32;

pub type BlockData = [[[BlockID; CHUNKSIZE]; CHUNKSIZE]; CHUNKSIZE];

#[derive(Clone, Copy)]
pub struct Chunk {
    pub data: BlockData,
}
