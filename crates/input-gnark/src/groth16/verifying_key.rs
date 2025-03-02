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
    use ark_bn254::Bn254;
    use ark_groth16::Groth16;

    use crate::groth16::{Proof, PublicInputs};

    use super::*;

    #[test]
    fn test_verifying_key() {
        let bytes = hex::decode("a0b0ee562d81cf259f9e1297c255eb270743355a25058dbc294fba918bb1223ace700f2ae0553f5465ef2159172f69f952a70bef66e007ce949cf744ac26bdd9cc8c1e7acb1de4d6bf520ebc1c5651c6cbf47d1796efec861895ab1d9cacc1831416a4b34670f83f328bd8bc393fcad208a0dcad6c8efa923cbc8944fd76f476a84fbc81a191bbf5730796fb81e4092067603a5d4cb885d2542c67f17a12408a15fcf3b660cd41edb782e7575749816065626e970a7fe31ce0d15a44e810af48ef3ddff940ce5c7374c31d5b36bebb3deead87dda4ed1b5d94d9ffb7d5908aaadcb761704bae1dc5d247edf180cdc53860b12400788bd8ca0e6db962c6e60e020d80ba4c77a7c8ceb17442412bad6d6ff0bfb68cfa6552587ecb9efde0cebdf600000002c2b7148d01b2cd59440d0b5c6cb0761d00c9a4fd62e9f6ae84cc5bb1ed8ade27ede46df18d743e6bcf37a4a3b2e915b22635b12b782800fd71e58a5de6e056f10000000000000000").unwrap();
        let vk = VerifyingKey::from_gnark_compressed_bytes(&bytes).unwrap();

        let vk = vk.into_core_type().unwrap();
        println!("vk: {:#?}", vk);
    }

    #[test]
    fn test_verify() {
        let bytes = hex::decode("e1a63b5b58ffa75197f1389ea0a1a0a2808cf42770257cbff750c9d085f14889a1c8e8ce5edabfe86507e9cac3f91a02d1bea91d32f5de992777cff55b67d5e5e0460cebb19f25a19debd1298158210f7ca4d0e8f43008fb73c86ea4cbc2f3f82633a327e117a2761c0bf0e6ae7795b5286d700f8c2b328dcf3a7899544e0ae5efab89be05bc46c081b24f748da6b5db23628cdcf697f9db431312fd456b58e805bf842b5cf976afff93d293cbfc619b3d181319edeb9f53a90c932a72363733e45ea9262164155e7099b10a135cb0a42087d1aa52d5c35e716cfc80915ddff3c6d28eb17c5f4aff3081a2af8f1099b8c23588d728111b0c30f72f732425dc691ef7f1ffe8eafdd59683b2ed502e7c1127d004bf2239803e1143c55e582fbe1800000002ad1290a3cf0a0dbd8231016326b20fd33697fbd70dba86286917db723e880fdc8685e1da7adf55b9b2edb4dc42cedbe030d16b048b0f9a40ebaba36d558a171c0000000000000000").unwrap();
        let vk = VerifyingKey::from_gnark_compressed_bytes(&bytes).unwrap();
        let vk: groth16::VerifyingKey = vk.into_core_type().unwrap();
        let vk = vk.to_arkworks();
        println!("vk: {:#?}", vk);

        let bytes = hex::decode("8bb91240454d63a7b9d6641e7788759a3bb7052e424c37f7258e94c65f84164de7aa63ae5768ffbfbe22688751456e6414ec270b31d8ebb238fafcb8db99cba7223c6b4775d6b8a33a0a44c2da947b9241fe17abbfa43dafd987f79112543b18ea01274a97a09d65009bfa37820fa8df560566a927ba7c2f9340043420afb777000000004000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let proof = Proof::from_gnark_compressed_bytes(&bytes).unwrap();
        let proof = proof.into_core_type().unwrap();
        let proof = proof.to_arkworks();

        let bytes = hex::decode("0000000100000000000000010000000000000000000000000000000000000000000000000000000000000021").unwrap();
        let public_inputs = PublicInputs::from_gnark_compressed_bytes(&bytes).unwrap();
        let public_inputs = public_inputs.into_core_type().unwrap();
        let public_inputs = public_inputs.to_arkworks();

        let pvk = ark_groth16::prepare_verifying_key(&vk);
        let result = Groth16::<Bn254>::verify_proof(&pvk, &proof, &public_inputs).unwrap();
        println!("result: {}", result);
    }
}
