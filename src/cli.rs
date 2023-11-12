use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version = env!("CARGO_PKG_VERSION"), author = "Manuel Huber")]
pub struct Cli {
    #[cfg(all(feature = "ftd2", feature = "ftdi"))]
    /// bla bla bla
    #[arg(value_enum, short = 'I', long)]
    implementation: FtdiLib,

    /// Vendor ID to search for
    #[arg(long, default_value = "0x0403")]
    pub vid: String,

    /// Product ID to search for
    #[arg(long, default_value = "0x6001")]
    pub pid: String,
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
        compile_error!("Either ftdi or ftd2 has to be enabled (or both)");
    }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum FtdiLib {
    #[value(name = "1")]
    Ftdi1,
    #[value(name = "2")]
    Ftd2,
}
