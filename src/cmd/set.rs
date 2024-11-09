use anyhow::Result;

use crate::cmd::{Init, Run};

impl Run for Init {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
