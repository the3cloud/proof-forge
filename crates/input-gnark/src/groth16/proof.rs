use anyhow::{anyhow, Result};
use proof_forge_core::groth16;

use crate::curve::bn254::{G1Point, G2Point};

#[derive(Debug)]
pub struct Proof {
    pub a: G1Point,
    pub b: G2Point,
    pub c: G1Point,
}

impl Proof {
    pub fn from_gnark_compressed_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 128 {
            return Err(anyhow!("Invalid proof length"));
        }

        let a = G1Point::from_gnark_compressed_bytes(&bytes[0..32])?;
        let b = G2Point::from_gnark_compressed_bytes(&bytes[32..96])?;
        let c = G1Point::from_gnark_compressed_bytes(&bytes[96..128])?;

        Ok(Self { a, b, c })
    }

    pub fn into_core_type(self) -> Result<groth16::Proof> {
        Ok(groth16::Proof {
            curve: proof_forge_core::ZKProofCurve::BN254,
            a: self.a.into_core_type()?,
            b: self.b.into_core_type()?,
            c: self.c.into_core_type()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof() {
        let bytes = hex::decode("e8464599fca324e9e4874d131db4b1ada8128a3ce31e407e16d78b40bc8b67bba63d15871aa20c1dc79083f0b4f7a4776380729f2bae11afa43e125150434afa282c6b1a8bd8d668600ea1a9ded131fbd54d4023b8c11d4861514336af629c47ef2b937cb569be6712c2875b4e602cbae12d982d6efcc3680d6037f16cf6576f000000004000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let proof = Proof::from_gnark_compressed_bytes(&bytes).unwrap();

        println!("proof: {:#?}", proof);
    }
}
