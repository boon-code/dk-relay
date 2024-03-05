use crate::relay::{BasicOps, BitBangDev};
use crate::Result;
use crate::{cli::Cli, relay::InitDeinit};
use anyhow::bail;
use ftdi::{self, Device};
use std::{
    io::{Read, Write},
    time::Duration,
};

pub struct Ftdi1Dev {
    dev: Option<Device>,
}
impl Ftdi1Dev {
    pub fn new(out_mask: &[u8]) -> Result<Self> {
        let mut s = Self { dev: None };
        s.init(out_mask)?;
        Ok(s)
    }
}
impl BasicOps for Ftdi1Dev {
    fn write_pins(&mut self, buf: &mut [u8]) -> Result<()> {
        if let Some(dev) = self.dev.as_mut() {
            dev.write_all(buf)?;
            Ok(())
        } else {
            bail!("Not initialized");
        }
    }

    fn read_pins(&mut self, buf: &mut [u8]) -> Result<()> {
        if let Some(dev) = self.dev.as_mut() {
            dev.read_exact(buf)?;
            Ok(())
        } else {
            bail!("Not initialized");
        }
    }
}
impl InitDeinit for Ftdi1Dev {
    fn init(&mut self, out_mask: &[u8]) -> Result<()> {
        let vid: u16 = 0x0403;
        let pid: u16 = 0x6001;
        if self.dev.is_some() {
            bail!("Device is already initialized");
        }
        if let Some(mask) = out_mask.first() {
            _ = self.dev.take();
            let mut dev = ftdi::find_by_vid_pid(vid, pid).open()?;
            dev.set_bitmode(*mask, ftdi::BitMode::SyncBB)?;
            self.dev.replace(dev);
            Ok(())
        } else {
            bail!("Output mask is an empty array");
        }
    }

    fn deinit(&mut self) -> Result<()> {
        _ = self.dev.take();
        Ok(())
    }
}
impl BitBangDev for Ftdi1Dev {}
impl Drop for Ftdi1Dev {
    fn drop(&mut self) {
        _ = self.deinit(); // TODO: log if this errors
    }
}

pub fn demo(cli: &Cli) -> Result<()> {
    let timeout = Duration::from_millis(100);
    let vid: u16 = 0x0403;
    let pid: u16 = 0x6001;
    println!("FTDI1");
    let mut dev = ftdi::find_by_vid_pid(vid, pid)
        //.serial("DAE005CV")
        .set_detach_mode(ftdi::DetachMode::AutoDetachReatach);
    if let Some(serial) = &cli.serial {
        dev = dev.serial(serial);
    }
    let mut dev = dev.open()?;
    println!("Type: {:?}", unsafe { (*dev.libftdi_context()).type_ });
    println!("Interface: {:?}", unsafe {
        (*dev.libftdi_context()).interface
    });
    println!("Bitbang mode: {:?}", unsafe {
        (*dev.libftdi_context()).bitbang_mode
    });
    println!("Bitbang enabled: {:?}", unsafe {
        (*dev.libftdi_context()).bitbang_enabled
    });
    println!("Detach mode: {:?}", unsafe {
        dev.libftdi_context().read().module_detach_mode
    });
    println!("usb_dev: {:?}", unsafe {
        dev.libftdi_context().read().usb_dev
    });
    println!("Detach mode: {:?}", unsafe {
        dev.libftdi_context().read().module_detach_mode
    });

    if cli.only_display {
        return Ok(());
    }

    dev.set_bitmode(0xff, ftdi::BitMode::SyncBB)?;
    let mut buf: [u8; 1] = [0; 1];
    dev.read(&mut buf)?;

    println!("{buf:?}");

    buf[0] = 0;

    for i in 0..8 {
        println!("Enabling bit {i}");
        buf[0] = 1 << i;
        dev.write(&mut buf)?;

        dev.read(&mut buf)?;
        println!("{buf:?}");

        std::thread::sleep(timeout);
    }

    buf[0] = 0;
    dev.write(&mut buf)?;
    std::thread::sleep(Duration::from_secs(1));

    println!("Christmas tree...");
    let mut state: u8 = 0;
    for i in 0..8 {
        println!("Adding bit {i}");
        state |= 1 << i;
        buf[0] = state;
        dev.write(&mut buf)?;

        dev.read(&mut buf)?;
        println!("{buf:?}");

        std::thread::sleep(timeout);
    }

    println!("Resetting");

    buf[0] = 0xff;
    dev.write(&mut buf)?;

    Ok(())
}
