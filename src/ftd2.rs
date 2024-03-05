use std::time::Duration;
use crate::Result;
use libftd2xx::{Ftdi, FtdiCommon};
use crate::cli::Cli;

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
