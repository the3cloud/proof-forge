use alloy_primitives::U256;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G1Point {
    pub x: U256,
    pub y: U256,
}

impl G1Point {
    pub fn from_oct_str(x: &str, y: &str) -> Result<Self> {
        let x: U256 = x.parse()?;
        let y: U256 = y.parse()?;

        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct G2Point {
    pub x0: U256,
    pub x1: U256,
    pub y0: U256,
    pub y1: U256,
}

impl G2Point {
    pub fn from_oct_str(x0: &str, x1: &str, y0: &str, y1: &str) -> Result<Self> {
        let x0: U256 = x0.parse()?;
        let x1: U256 = x1.parse()?;
        let y0: U256 = y0.parse()?;
        let y1: U256 = y1.parse()?;

        Ok(Self { x0, x1, y0, y1 })
    }
}
