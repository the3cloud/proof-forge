use anyhow::{anyhow, Result};
use proof_forge_core::groth16;

use crate::curve::bn254::{G1Point, G2Point};

#[derive(Debug)]
pub struct VerifyingKey {
    pub alpha_g1: G1Point,
    pub beta_g2: G2Point,
    pub gamma_g2: G2Point,
    pub delta_g2: G2Point,
    pub ic: Vec<G1Point>,
}

impl VerifyingKey {
    pub fn from_gnark_compressed_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 292 {
            return Err(anyhow!("Invalid verifying key length"));
        }

        let ic_len = u32::from_be_bytes([bytes[288], bytes[289], bytes[290], bytes[291]]);

        if bytes.len() < 292 + ic_len as usize * 32 {
            return Err(anyhow!("Invalid IC length"));
        }

        let alpha_g1 = G1Point::from_gnark_compressed_bytes(&bytes[0..32])?;
        let beta_g2 = G2Point::from_gnark_compressed_bytes(&bytes[64..128])?;
        let gamma_g2 = G2Point::from_gnark_compressed_bytes(&bytes[128..192])?;
        let delta_g2 = G2Point::from_gnark_compressed_bytes(&bytes[224..288])?;

        let mut ic = Vec::new();

        for i in 0..ic_len {
            let begin = 292 + i as usize * 32;
            let end = begin + 32;

            let bytes = &bytes[begin..end];

            let ic_point = G1Point::from_gnark_compressed_bytes(bytes)?;
            ic.push(ic_point);
        }

        Ok(Self {
            alpha_g1,
            beta_g2,
            gamma_g2,
            delta_g2,
            ic,
        })
    }

    pub fn into_core_type(self) -> Result<groth16::VerifyingKey> {
        let alpha_g1 = self.alpha_g1.into_core_type()?;
        let beta_g2 = self.beta_g2.into_core_type()?;
        let gamma_g2 = self.gamma_g2.into_core_type()?;
        let delta_g2 = self.delta_g2.into_core_type()?;
        let ic = self
            .ic
            .into_iter()
            .map(|p| p.into_core_type())
            .collect::<Result<Vec<_>>>()?;

        Ok(groth16::VerifyingKey {
            curve: proof_forge_core::ZKProofCurve::BN254,
            alpha: alpha_g1,
            beta: beta_g2,
            gamma: gamma_g2,
            delta: delta_g2,
            ic,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifying_key() {
        let bytes = hex::decode("a0b0ee562d81cf259f9e1297c255eb270743355a25058dbc294fba918bb1223ace700f2ae0553f5465ef2159172f69f952a70bef66e007ce949cf744ac26bdd9cc8c1e7acb1de4d6bf520ebc1c5651c6cbf47d1796efec861895ab1d9cacc1831416a4b34670f83f328bd8bc393fcad208a0dcad6c8efa923cbc8944fd76f476a84fbc81a191bbf5730796fb81e4092067603a5d4cb885d2542c67f17a12408a15fcf3b660cd41edb782e7575749816065626e970a7fe31ce0d15a44e810af48ef3ddff940ce5c7374c31d5b36bebb3deead87dda4ed1b5d94d9ffb7d5908aaadcb761704bae1dc5d247edf180cdc53860b12400788bd8ca0e6db962c6e60e020d80ba4c77a7c8ceb17442412bad6d6ff0bfb68cfa6552587ecb9efde0cebdf600000002c2b7148d01b2cd59440d0b5c6cb0761d00c9a4fd62e9f6ae84cc5bb1ed8ade27ede46df18d743e6bcf37a4a3b2e915b22635b12b782800fd71e58a5de6e056f10000000000000000").unwrap();
        let vk = VerifyingKey::from_gnark_compressed_bytes(&bytes).unwrap();

        let vk = vk.into_core_type().unwrap();
        println!("vk: {:#?}", vk);
    }
}
