use clap::{Parser, ValueEnum};


#[cfg(not(any(feature = "ftdi", feature = "ftd2")))]
compile_error!("Either ftdi or ftd2 has to be enabled (or both)");

#[derive(Parser, Debug)]
#[command(version = env!("CARGO_PKG_VERSION"), author = "Manuel Huber")]
pub struct Cli {
    #[cfg(all(feature = "ftd2", feature = "ftdi"))]
    /// library to use
    #[arg(value_enum, short = 'I', long)]
    implementation: FtdiLib,

    /// Vendor ID to search for
    #[arg(long, default_value = "0x0403")]
    pub vid: String,

    /// Product ID to search for
    #[arg(long, default_value = "0x6001")]
    pub pid: String,

    /// Serial number
    #[arg(short = 's', long)]
    pub serial: Option<String>,
}
impl Cli {
    pub fn get_lib(&self) -> FtdiLib {
        #[cfg(all(feature = "ftd2", feature = "ftdi"))] {
            return self.implementation.clone();
        }

        #[cfg(all(feature = "ftd2", not(feature = "ftdi")))] {
            return FtdiLib::Ftd2;
        }

        #[cfg(all(feature = "ftdi", not(feature = "ftd2")))] {
            return FtdiLib::Ftdi1;
        }

        #[cfg(not(any(feature = "ftdi", feature = "ftd2")))]
        unreachable!("Either ftdi or ftd2 has to be enabled");
    }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum FtdiLib {
    #[cfg(feature = "ftdi")]
    #[value(name = "1")]
    Ftdi1,

    #[cfg(feature = "ftd2")]
    #[value(name = "2")]
    Ftd2,
}
