use anyhow::Result;
use dk_relay;


fn main() -> Result<()> {
    dk_relay::demo()?;
    Ok(())
}
