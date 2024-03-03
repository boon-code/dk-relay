use anyhow::Result;
use clap::Parser;

#[cfg(feature = "ftdi")]
mod ftdi1;

#[cfg(feature = "ftd2")]
mod ftd2;

mod cli;

mod relay;

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
