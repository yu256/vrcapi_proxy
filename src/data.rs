use crate::general::write_json;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub auth: String,
    pub token: String,
    pub askme: bool,
}

pub(crate) trait DataVecExt {
    fn write(&self) -> Result<()>;
}

impl DataVecExt for Vec<Data> {
    fn write(&self) -> Result<()> {
        write_json(self, "data")?;
        Ok(())
    }
}
