use alloy_primitives::U256;
use anyhow::{anyhow, Result};
use ark_bn254::{G1Affine, G2Affine};
use ark_ec::AffineRepr;
use ark_ff::{BigInteger, PrimeField};
use ark_serialize::CanonicalDeserialize;

use crate::utils;

#[derive(Debug)]
pub struct G1Point {
    pub x: G1Affine,
}

impl G1Point {
    pub fn from_gnark_compressed_bytes(x: &[u8]) -> Result<Self> {
        if x.len() != 32 {
            return Err(anyhow!("Invalid G1 point length"));
        }

        let mut bytes = x.to_vec();
        bytes[0] = utils::gnark_flag_to_ark_flag(bytes[0])?;
        bytes.reverse();

        let x = G1Affine::deserialize_compressed(&mut bytes.as_slice())
            .map_err(|e| anyhow!("Failed to deserialize G1 point: {}", e))?;
        Ok(Self { x })
    }

    pub fn into_core_type(self) -> Result<proof_forge_core::G1Point> {
        let (x, y) = self.x.xy().ok_or(anyhow!("Invalid G1 point"))?;

        let x = x.into_bigint().to_bytes_be();
        let x = U256::from_be_slice(&x);

        let y = y.into_bigint().to_bytes_be();
        let y = U256::from_be_slice(&y);

        Ok(proof_forge_core::G1Point { x, y })
    }
}

#[derive(Debug)]
pub struct G2Point {
    pub x: G2Affine,
}

impl G2Point {
    pub fn from_gnark_compressed_bytes(x: &[u8]) -> Result<Self> {
        let mut bytes = x.to_vec();
        bytes[0] = utils::gnark_flag_to_ark_flag(bytes[0])?;
        bytes.reverse();

        let x = G2Affine::deserialize_compressed(&mut bytes.as_slice())
            .map_err(|e| anyhow!("Failed to deserialize G2 point: {}", e))?;
        Ok(Self { x })
    }

    pub fn into_core_type(self) -> Result<proof_forge_core::G2Point> {
        let (x, y) = self.x.xy().ok_or(anyhow!("Invalid G1 point"))?;

        let x0 = x.c1.into_bigint().to_bytes_be();
        let x0 = U256::from_be_slice(&x0);

        let x1 = x.c0.into_bigint().to_bytes_be();
        let x1 = U256::from_be_slice(&x1);

        // let q: U256 =
        //     "21888242871839275222246405745257275088696311157297823662689037894645226208583"
        //         .parse()?;

        let y0 = y.c1.into_bigint().to_bytes_be();
        let y0 = U256::from_be_slice(&y0);
        // let y0 = (q - y0) % q;

        let y1 = y.c0.into_bigint().to_bytes_be();
        let y1 = U256::from_be_slice(&y1);
        // let y1 = (q - y1) % q;

        Ok(proof_forge_core::G2Point { x0, y0, x1, y1 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g1_point() {
        let alpha_bytes =
            hex::decode("9405cb09ba747e544fb2123853b2f345240cd857068b858c451b31a183967187")
                .unwrap();

        let alpha = G1Point::from_gnark_compressed_bytes(&alpha_bytes).unwrap();
        let alpha_core = alpha.into_core_type().unwrap();

        let x: U256 =
            "9056492523773147063857211398509875118128187246236322079553712688092669178247"
                .parse()
                .unwrap();

        assert_eq!(alpha_core.x, x);

        let y: U256 =
            "9616391677916058273867466400291683407229826057663778558522063618030201158179"
                .parse()
                .unwrap();

        assert_eq!(alpha_core.y, y);
    }

    #[test]
    fn test_g2_point() {
        let beta_bytes = hex::decode(
            "abf0fd2a29dd79694d244ca151cd5314311861e366a94f7d169502d0a707c6b017d03cabc57d6c35aa2e75d15d19eb45acafaa6d2b5b67aa0e2a2a636c3dec70",
        )
        .unwrap();

        let beta = G2Point::from_gnark_compressed_bytes(&beta_bytes).unwrap();
        let beta_core = beta.into_core_type().unwrap();

        println!("beta_core: {:?}", beta_core);
    }
}
