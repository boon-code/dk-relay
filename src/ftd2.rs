use crate::cli::Cli;
use crate::relay::{BasicOps, BitBangDev, InitDeinit};
use crate::Result;
use anyhow::bail;
use libftd2xx::{Ftdi, FtdiCommon};
use std::time::Duration;

pub struct Ftd2Dev {
    is_init: bool,
    dev: Ftdi,
}
impl Ftd2Dev {
    pub fn new() -> Result<Self> {
        let dev = Ftdi::new()?;
        Ok(Self {
            is_init: false,
            dev,
        })
    }

    pub fn new_init(out_mask: &[u8]) -> Result<Self> {
        let mut dev = Self::new()?;
        dev.init(out_mask)?;
        Ok(dev)
    }

    fn ensure_is_init(&self) -> Result<()> {
        if self.is_init {
            Ok(())
        } else {
            bail!("Device is not initialized");
        }
    }
}
impl BasicOps for Ftd2Dev {
    fn write_pins(&mut self, buf: &mut [u8]) -> Result<()> {
        self.ensure_is_init()?;
        self.dev.write_all(buf)?;
        Ok(())
    }

    fn read_pins(&mut self, buf: &mut [u8]) -> Result<()> {
        self.ensure_is_init()?;
        self.dev.read_all(buf)?;
        Ok(())
    }
}
impl InitDeinit for Ftd2Dev {
    fn init(&mut self, out_mask: &[u8]) -> Result<()> {
        if self.is_init {
            bail!("Device is already initialized");
        }
        if let Some(mask) = out_mask.first() {
            self.dev
                .set_bit_mode(*mask, libftd2xx::BitMode::SyncBitbang)?;
            self.is_init = true;
            Ok(())
        } else {
            bail!("Output mask is an empty array");
        }
    }

    fn deinit(&mut self) -> Result<()> {
        if self.is_init {
            self.is_init = false;
        }
        Ok(())
    }
}
impl Drop for Ftd2Dev {
    fn drop(&mut self) {
        let _ = self.deinit(); // TODO: log if this errors
    }
}
impl BitBangDev for Ftd2Dev {}

pub fn demo(cli: &Cli) -> Result<()> {
    println!("FTD2");
    let mut ft = cli.serial.as_ref()
        .map(|serial| Ftdi::with_serial_number(serial.as_str()))
        .unwrap_or_else(|| Ftdi::new())?;
    let info = ft.device_info()?;
    println!("Device information: {:?}", info);

    if cli.only_display {
        return Ok(());
    }

    ft.set_bit_mode(0xff, libftd2xx::BitMode::SyncBitbang)?;

    let mut buf: [u8; 1] = [0; 1];
    buf[0] = 0x00;
    ft.write(&buf)?;

    std::thread::sleep(Duration::from_secs(1));

    for i in 0_u8..8 {
        buf[0] = 1 << i;
        ft.write(&buf)?;

        std::thread::sleep(Duration::from_millis(200));
    }

    buf[0] = 0x00;
    ft.write(&buf)?;

    std::thread::sleep(Duration::from_secs(1));

    for i in 0_u8..8 {
        buf[0] |= 1 << i;
        ft.write(&buf)?;

        std::thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}
