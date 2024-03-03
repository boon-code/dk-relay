// re-exports
mod basic;
pub mod ops8bit;

pub use basic::{BasicOps, InitDeinit};

pub trait BitBangDev: BasicOps + InitDeinit {}

pub type DynRelay = Box<dyn BitBangDev>;

#[cfg(test)]
mod tests;
