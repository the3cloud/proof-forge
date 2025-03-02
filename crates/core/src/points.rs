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

    #[cfg(feature = "arkworks")]
    pub fn to_arkworks(&self) -> ark_bn254::G1Affine {
        use ark_ff::PrimeField;

        let x_point = ark_bn254::Fq::from_le_bytes_mod_order(&self.x.to_le_bytes_vec());
        let y_point = ark_bn254::Fq::from_le_bytes_mod_order(&self.y.to_le_bytes_vec());

        ark_bn254::G1Affine::new(x_point, y_point)
    }

    #[cfg(feature = "arkworks")]
    pub fn from_arkworks(point: ark_bn254::G1Affine) -> Self {
        use ark_ff::{BigInteger, PrimeField};

        let x = U256::from_be_slice(&point.x.into_bigint().to_bytes_be());
        let y = U256::from_be_slice(&point.y.into_bigint().to_bytes_be());

        Self { x, y }
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

    #[cfg(feature = "arkworks")]
    pub fn to_arkworks(&self) -> ark_bn254::G2Affine {
        use ark_bn254::{Fq, Fq2, G2Affine};
        use ark_ff::PrimeField;

        let x0_point = Fq::from_le_bytes_mod_order(&self.x0.to_le_bytes_vec());
        let x1_point = Fq::from_le_bytes_mod_order(&self.x1.to_le_bytes_vec());

        let x = Fq2 {
            c0: x1_point,
            c1: x0_point,
        };

        let y0_point = Fq::from_le_bytes_mod_order(&self.y0.to_le_bytes_vec());
        let y1_point = Fq::from_le_bytes_mod_order(&self.y1.to_le_bytes_vec());

        let y = Fq2 {
            c0: y1_point,
            c1: y0_point,
        };

        G2Affine::new(x, y)
    }

    #[cfg(feature = "arkworks")]
    pub fn from_arkworks(point: ark_bn254::G2Affine) -> Result<Self> {
        use ark_ec::AffineRepr;
        use ark_ff::{BigInteger, PrimeField};

        let (x, y) = point.xy().ok_or(anyhow::anyhow!("Invalid G2 point"))?;

        let x0 = U256::from_be_slice(&x.c1.into_bigint().to_bytes_be());
        let x1 = U256::from_be_slice(&x.c0.into_bigint().to_bytes_be());
        let y0 = U256::from_be_slice(&y.c1.into_bigint().to_bytes_be());
        let y1 = U256::from_be_slice(&y.c0.into_bigint().to_bytes_be());

        Ok(Self { x0, x1, y0, y1 })
    }
}
