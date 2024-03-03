use super::*;
use crate::Result;
use super::ops8bit::IoOps8Bit;

struct TestDevice {
    pins: u8,
}
impl TestDevice {
    fn new() -> Result<Self> {
        Self::new_with(0_u8)
    }

    fn new_with(pins: u8) -> Result<Self> {
        let mut x = Self { pins };
        x.init(&[0xff_u8])?;
        Ok(x)
    }

    fn value(&self) -> u8 {
        self.pins
    }
}
impl BasicOps for TestDevice {
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
impl InitDeinit for TestDevice {
    fn init(&mut self, _out_mask: &[u8]) -> Result<()> {
        Ok(())
    }
}
impl BitBangDev for TestDevice {}

#[test]
fn test_set_pins() {
    let mut dev = TestDevice::new().unwrap();
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
    let mut dev = TestDevice::new_with(255_u8).unwrap();
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
    let mut dev = TestDevice::new().unwrap();
    dev.get_pin(8).unwrap_err();
    dev.set_pin(8, true).unwrap_err();
    dev.set_pin(8, false).unwrap_err();
}

#[test]
fn test_no_change_set() {
    let mut dev = TestDevice::new_with(0b10100101).unwrap();
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
    let mut dev = TestDevice::new_with(0b10100101).unwrap();
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
    let mut dk = TestDevice::new()?;
    handle_stuff(&mut dk);
    Ok(())
}

fn handle_stuff<T: BitBangDev>(dk: &mut T) {
    // nop
}
