use anyhow::{bail, Result};

pub trait BasicOps {
    fn write_pins(&mut self, buf: &mut [u8]) -> Result<()>;
    fn read_pins(&mut self, buf: &mut [u8]) -> Result<()>;
}
impl<T> IoOps for T where T: BasicOps {}

pub trait InitDeinit {
    fn init(&mut self, out_mask: u8) -> Result<()>;

    fn deinit(&mut self) -> Result<()> {
        Ok(())
    }
}

pub trait IoOps: BasicOps {
    fn read8_pins(&mut self) -> Result<u8> {
        let mut buf: [u8; 1] = [0; 1];
        self.read_pins(&mut buf[..])?;
        Ok(buf[0])
    }
    fn write8_pins(&mut self, pins: u8) -> Result<()> {
        let mut buf = [pins; 1];
        self.write_pins(&mut buf)?;
        Ok(())
    }

    fn get_pin(&mut self, index: u8) -> Result<bool> {
        if index > 7 {
            bail!("Index is out of bounds: {index}");
        }
        let val = self.read8_pins()?;
        let mask = 1 << index;

        Ok(val & mask == mask)
    }

    fn set_pin(&mut self, index: u8, val: bool) -> Result<()> {
        if index > 7 {
            bail!("Index is out of bounds: {index}");
        }
        let pins = self.read8_pins()?;
        let mask = 1 << index;
        let bits = if val { pins | mask } else { pins & (!mask) };
        self.write8_pins(bits)?;
        Ok(())
    }
}

pub trait BitBangDev: BasicOps + InitDeinit {}

pub struct Relay {}
impl InitDeinit for Relay {
    fn init(&mut self, out_mask: u8) -> Result<()> {
        Ok(())
    }
}
impl Drop for Relay {
    fn drop(&mut self) {
        _ = self.deinit();
    }
}
impl BasicOps for Relay {
    fn write_pins(&mut self, buf: &mut [u8]) -> Result<()> {
        Ok(())
    }

    fn read_pins(&mut self, buf: &mut [u8]) -> Result<()> {
        Ok(())
    }
}
impl BitBangDev for Relay {}
impl Relay {
    pub fn new() -> Result<Self> {
        let mut s = Self {};
        s.init(8)?;
        Ok(s)
    }
}

mod test {
    use super::{BitBangDev, IoOps, Relay, Result};

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
}
