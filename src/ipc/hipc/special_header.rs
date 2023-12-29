use zerocopy_derive::{FromBytes, FromZeroes, AsBytes};

#[derive(Copy, Clone, Default, Debug, FromBytes, FromZeroes, AsBytes)]
#[repr(C, packed)]
pub struct SpecialHeader {
    pub block: u32,
}

impl SpecialHeader {
    pub fn has_pid(&self) -> bool { (self.block & 0b1) != 0 }
    pub fn copy_handle_count(&self) -> u8 { ((self.block >> 1) & 0xF) as u8 }
    pub fn move_handle_count(&self) -> u8 { ((self.block >> 5) & 0xF) as u8 }

    pub fn set_has_pid(&mut self, pid: bool) {
        self.block &= 0xFFFFFFFE;
        self.block |= if pid { 1 } else { 0 }
    }

    pub fn set_copy_handle_count(&mut self, copy_handle_count: u8) {
        if copy_handle_count > 0xF {
            panic!("copy_handle_count too high! max is 0xF, got: {}", copy_handle_count);
        }
        self.block &= 0xFFFFFFE1;
        self.block |= (copy_handle_count as u32) << 1;
    }

    pub fn set_move_handle_count(&mut self, move_handle_count: u8) {
        if move_handle_count > 0xF {
            panic!("move_handle_count too high! max is 0xF, got: {}", move_handle_count);
        }

        self.block &= 0xFFFFFE1F;
        self.block |= (move_handle_count as u32) << 5;
    }

    pub fn with_has_pid(mut self, has_pid: bool) -> Self { self.set_has_pid(has_pid); self }
    pub fn with_copy_handle_count(mut self, copy_handle_count: u8) -> Self { self.set_copy_handle_count(copy_handle_count); self }
    pub fn with_move_handle_count(mut self, move_handle_count: u8) -> Self { self.set_move_handle_count(move_handle_count); self }

    pub fn new(has_pid: bool, copy_handle_count: u8, move_handle_count: u8) -> Self {
        Self::default()
            .with_has_pid(has_pid)
            .with_copy_handle_count(copy_handle_count)
            .with_move_handle_count(move_handle_count)
    }
}
