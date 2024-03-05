use crate::relay::{BitBangDev, BasicOps, InitDeinit};
use crate::Result;
use super::IoOps8Bit;

#[test]
fn testit() {
    let mut r = Relay::new().unwrap();
    dosome(&mut r).unwrap();
}

fn dosome<T: BitBangDev>(t: &mut T) -> Result<()> {
    //use super::IoOps;
    t.get_pin(5)?;
    Ok(())
}

pub struct Relay {}
impl InitDeinit for Relay {
    fn init(&mut self, _out_mask: &[u8]) -> Result<()> {
        Ok(())
    }
}
impl Drop for Relay {
    fn drop(&mut self) {
        _ = self.deinit();
    }
}
impl BasicOps for Relay {
    fn write_pins(&mut self, _buf: &mut [u8]) -> Result<()> {
        Ok(())
    }

    fn read_pins(&mut self, _buf: &mut [u8]) -> Result<()> {
        Ok(())
    }
}
impl BitBangDev for Relay {}
impl Relay {
    pub fn new() -> Result<Self> {
        let mut s = Self {};
        s.init(&[8])?;
        Ok(s)
    }
}
