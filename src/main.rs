use anyhow::Result;
use ftdi;
use std::{io::{Read, Write}, time::Duration};


fn main() -> Result<()> {
    let timeout = Duration::from_millis(100);
    let vid :u16 = 0x0403;
    let pid :u16 = 0x6001;
    let mut dev = ftdi::find_by_vid_pid(vid, pid)
        //.serial("DAE005CV")
        .set_detach_mode(ftdi::DetachMode::AutoDetachReatach)
        .open()?;
    dev.set_bitmode(0xff, ftdi::BitMode::SyncBB)?;
    println!("Type: {:?}", unsafe { (*dev.libftdi_context()).type_ });
    println!("Interface: {:?}", unsafe { (*dev.libftdi_context()).interface });
    println!("Bitbang mode: {:?}", unsafe { (*dev.libftdi_context()).bitbang_mode });
    println!("Bitbang enabled: {:?}", unsafe { (*dev.libftdi_context()).bitbang_enabled });
    println!("Detach mode: {:?}", unsafe { dev.libftdi_context().read().module_detach_mode });
    println!("usb_dev: {:?}", unsafe { dev.libftdi_context().read().usb_dev });
    println!("Detach mode: {:?}", unsafe { dev.libftdi_context().read().module_detach_mode });

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
