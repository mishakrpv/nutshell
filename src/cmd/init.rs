use anyhow::Result;

use crate::cmd::{Run, Set};

impl Run for Set {
    fn run(&self) -> Result<()> {
        !unimplemented!()
    }
}
