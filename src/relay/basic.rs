// Public traits and DkRelay struct
use crate::Result;

pub trait InitDeinit {
    fn init(&mut self, out_mask: &[u8]) -> Result<()>;

    fn deinit(&mut self) -> Result<()> {
        Ok(())
    }
}

pub trait BasicOps {
    fn write_pins(&mut self, buf: &mut [u8]) -> Result<()>;
    fn read_pins(&mut self, buf: &mut [u8]) -> Result<()>;
}

