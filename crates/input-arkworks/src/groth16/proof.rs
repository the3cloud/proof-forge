use anyhow::Result;
use ark_serialize::CanonicalDeserialize;
use proof_forge_core::groth16;

pub struct Proof {
    proof: ark_groth16::Proof<ark_bn254::Bn254>,
}

impl Proof {
    pub fn from_compressed_bytes(bytes: &[u8]) -> Result<Self> {
        let proof = ark_groth16::Proof::deserialize_compressed(bytes)?;

        Ok(Self { proof })
    }

    pub fn into_core_type(self) -> Result<groth16::Proof> {
        let core_type = groth16::Proof::from_arkworks(self.proof)?;

        Ok(core_type)
    }
}
