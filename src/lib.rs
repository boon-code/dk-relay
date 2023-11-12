use anyhow::Result;
use clap::Parser;

#[cfg(feature = "ftdi")]
mod ftdi1;

#[cfg(feature = "ftd2")]
mod ftd2;

pub mod cli;

pub fn demo() -> Result<()> {
    let cli = cli::Cli::parse();
    println!("Cli paramter: {cli:?}");

    match cli.get_lib() {
        cli::FtdiLib::Ftdi1 => { println!("FTDI v1") },
        cli::FtdiLib::Ftd2 => { println!("FTD2XX") },
    }

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
