use anyhow::Result;

#[cfg(feature = "ftdi")]
mod ftdi1;

#[cfg(feature = "ftd2")]
mod ftd2;

pub fn demo() -> Result<()> {
    #[cfg(feature = "ftdi")]
    let r1 = ftdi1::demo();

    #[cfg(feature = "ftd2")]
    let r2 = ftd2::demo();

    #[cfg(feature = "ftdi")]
    r1?;

    #[cfg(feature = "ftd2")]
    r2?;

    Ok(())
}
