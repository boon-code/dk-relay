// traits implementing operations using 8 bits only
// using the super trait defined in super::basic
use super::BasicOps;
use anyhow::{Result, bail};

pub trait IoOps8Bit: BasicOps {
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
impl<T> IoOps8Bit for T where T: BasicOps {}

#[cfg(test)]
mod tests;
