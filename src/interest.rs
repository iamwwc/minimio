use std::num::NonZeroU8;
const READABLE: u8 = 0b0_001;
const WRITEABLE: u8 = 0b0_010;
pub struct Interest(NonZeroU8);
impl Interest {
    pub fn is_readable(&self) -> bool {
        (self.0.get() & READABLE) != 0
    }
    pub fn is_writable(&self) -> bool {
        (self.0.get() & WRITEABLE) != 0
    }
}