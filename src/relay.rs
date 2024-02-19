use anyhow::{Result, bail};
use super::BitBangDevice;

pub struct DkRelay<T: BitBangDevice> {
    num: u8,
    dev: T,
}

impl<T: BitBangDevice> DkRelay<T> {

    pub fn new(num: u8, mut dev: T) -> Result<Self> {
        let mask = gen_mask_u8(num)?;
        dev.init(mask)?;
        Ok(Self { num , dev })
    }
}
impl<T: BitBangDevice> Drop for DkRelay<T> {
    fn drop(&mut self) {
        _ = self.dev.deinit();
    }
}

fn gen_mask_u8(num: u8) -> Result<u8> {
    if num > 8_u8 {
        bail!("A maximum of 8 relays is supported")
    }
    let mask = if num < 8 {
        (1_u8 << num) - 1_u8
    } else {
        0xff_u8
    };

    Ok(mask)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gen_mask_ok() {
        assert_eq!(0b_0000_0000_u8, gen_mask_u8(0).unwrap());
        assert_eq!(0b_0000_0001_u8, gen_mask_u8(1).unwrap());
        assert_eq!(0b_0000_0011_u8, gen_mask_u8(2).unwrap());
        assert_eq!(0b_0000_0111_u8, gen_mask_u8(3).unwrap());
        assert_eq!(0b_0000_1111_u8, gen_mask_u8(4).unwrap());
        assert_eq!(0b_0001_1111_u8, gen_mask_u8(5).unwrap());
        assert_eq!(0b_0011_1111_u8, gen_mask_u8(6).unwrap());
        assert_eq!(0b_0111_1111_u8, gen_mask_u8(7).unwrap());
        assert_eq!(0b_1111_1111_u8, gen_mask_u8(8).unwrap());
    }

    #[test]
    fn test_gen_mask_fail() {
        for i in 9..256 {
            gen_mask_u8(i as u8).unwrap_err();
        }
    }
}
