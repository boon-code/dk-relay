use std::time::Duration;

use libftd2xx::{Ftdi, FtdiCommon};
use anyhow::Result;

pub fn demo() -> Result<()> {
    println!("FTD2");
    let mut ft = Ftdi::new()?;
    let info = ft.device_info()?;
    println!("Device information: {:?}", info);

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
