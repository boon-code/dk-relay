use anyhow::{Result, bail};
use clap::Parser;

#[cfg(feature = "ftdi")]
mod ftdi1;

#[cfg(feature = "ftd2")]
mod ftd2;

pub mod cli;

mod relay;
pub use relay::DkRelay;

pub trait BitBangDevice {
    fn write_pins(&mut self, buf: &mut [u8]) -> Result<()>;
    fn read_pins(&mut self, buf: &mut [u8]) -> Result<()>;
    fn init(&mut self, out_mask: u8) -> Result<()>;

    fn deinit(&mut self) -> Result<()> {
        Ok(())
    }

    fn read8_pins(&mut self) -> Result<u8> {
        let mut buf :[u8; 1] = [0; 1];
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
        let bits = if val {
            pins | mask
        } else {
            pins & (!mask)
        };
        self.write8_pins(bits)?;
        Ok(())
    }
}

pub fn demo() -> Result<()> {
    let cli = cli::Cli::parse();
    println!("Cli paramter: {cli:?}");

    match cli.get_lib() {
        #[cfg(feature = "ftdi")]
        cli::FtdiLib::Ftdi1 => { println!("FTDI v1"); ftdi1::demo(&cli)?; },

        #[cfg(feature = "ftd2")]
        cli::FtdiLib::Ftd2 => { println!("FTD2XX"); ftd2::demo(&cli)?; },
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDevice {
        pins: u8,
    }
    impl TestDevice {
        fn new() -> Self {
            Self { pins: 0_u8 }
        }

        fn new_with(pins: u8) -> Self {
            Self { pins }
        }

        fn value(&self) -> u8 {
            self.pins
        }
    }
    impl BitBangDevice for TestDevice {

        fn init(&mut self, out_mask: u8) -> Result<()> {
            Ok(())
        }

        fn write_pins(&mut self, buf: &mut [u8]) -> Result<()> {
            assert_eq!(1, buf.len());
            self.pins = buf[0];
            Ok(())
        }

        fn read_pins(&mut self, buf: &mut [u8]) -> Result<()> {
            assert_eq!(1, buf.len());
            buf[0] = self.pins;
            Ok(())
        }
    }

    #[test]
    fn test_set_pins() {
        let mut dev = TestDevice::new();
        assert_eq!(0_u8, dev.value());
        assert_eq!(0_u8, dev.read8_pins().unwrap());
        dev.set_pin(0, true).unwrap();
        assert_eq!(1_u8, dev.value());
        assert_eq!(1_u8, dev.read8_pins().unwrap());
        dev.set_pin(1, true).unwrap();
        assert_eq!(3_u8, dev.value());
        assert_eq!(3_u8, dev.read8_pins().unwrap());
        dev.set_pin(2, true).unwrap();
        assert_eq!(7_u8, dev.value());
        assert_eq!(7_u8, dev.read8_pins().unwrap());
        dev.set_pin(3, true).unwrap();
        assert_eq!(15_u8, dev.value());
        assert_eq!(15_u8, dev.read8_pins().unwrap());
        dev.set_pin(4, true).unwrap();
        assert_eq!(31_u8, dev.value());
        assert_eq!(31_u8, dev.read8_pins().unwrap());
        dev.set_pin(5, true).unwrap();
        assert_eq!(63_u8, dev.value());
        assert_eq!(63_u8, dev.read8_pins().unwrap());
        dev.set_pin(6, true).unwrap();
        assert_eq!(127_u8, dev.value());
        assert_eq!(127_u8, dev.read8_pins().unwrap());
        dev.set_pin(7, true).unwrap();
        assert_eq!(255_u8, dev.value());
        assert_eq!(255_u8, dev.read8_pins().unwrap());
    }

    #[test]
    fn test_unset_pins() {
        let mut dev = TestDevice::new_with(255_u8);
        assert_eq!(255_u8, dev.value());
        assert_eq!(255_u8, dev.read8_pins().unwrap());
        dev.set_pin(0, false).unwrap();
        assert_eq!(254_u8, dev.value());
        assert_eq!(254_u8, dev.read8_pins().unwrap());
        dev.set_pin(1, false).unwrap();
        assert_eq!(252_u8, dev.value());
        assert_eq!(252_u8, dev.read8_pins().unwrap());
        dev.set_pin(2, false).unwrap();
        assert_eq!(248_u8, dev.value());
        assert_eq!(248_u8, dev.read8_pins().unwrap());
        dev.set_pin(3, false).unwrap();
        assert_eq!(240_u8, dev.value());
        assert_eq!(240_u8, dev.read8_pins().unwrap());
        dev.set_pin(4, false).unwrap();
        assert_eq!(224_u8, dev.value());
        assert_eq!(224_u8, dev.read8_pins().unwrap());
        dev.set_pin(5, false).unwrap();
        assert_eq!(192_u8, dev.value());
        assert_eq!(192_u8, dev.read8_pins().unwrap());
        dev.set_pin(6, false).unwrap();
        assert_eq!(128_u8, dev.value());
        assert_eq!(128_u8, dev.read8_pins().unwrap());
        dev.set_pin(7, false).unwrap();
        assert_eq!(0_u8, dev.value());
        assert_eq!(0_u8, dev.read8_pins().unwrap());
    }

    #[test]
    fn test_out_of_bounds() {
        let mut dev = TestDevice::new();
        dev.get_pin(8).unwrap_err();
        dev.set_pin(8, true).unwrap_err();
        dev.set_pin(8, false).unwrap_err();
    }

    #[test]
    fn test_no_change_set() {
        let mut dev = TestDevice::new_with(0b10100101);
        dev.set_pin(0, true).unwrap();
        assert_eq!(0xa5, dev.value());
        dev.set_pin(2, true).unwrap();
        assert_eq!(0xa5, dev.value());
        dev.set_pin(5, true).unwrap();
        assert_eq!(0xa5, dev.value());
        dev.set_pin(7, true).unwrap();
        assert_eq!(0xa5, dev.value());
    }

    #[test]
    fn test_no_change_unset() {
        let mut dev = TestDevice::new_with(0b10100101);
        dev.set_pin(1, false).unwrap();
        assert_eq!(0xa5, dev.value());
        dev.set_pin(3, false).unwrap();
        assert_eq!(0xa5, dev.value());
        dev.set_pin(4, false).unwrap();
        assert_eq!(0xa5, dev.value());
        dev.set_pin(6, false).unwrap();
        assert_eq!(0xa5, dev.value());
    }

    #[test]
    fn test_dk_relay_simple() -> Result<()> {
        let mut dk = DkRelay::new(8_u8, TestDevice::new())?;
        handle_stuff(&mut dk);
        Ok(())
    }

    fn handle_stuff<T: BitBangDevice>(dk: &mut DkRelay<T>) {
        // nop
    }
}
