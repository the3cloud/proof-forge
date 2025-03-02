use anyhow::Result;
use ark_bn254::Bn254;
use ark_serialize::CanonicalDeserialize;
use proof_forge_core::groth16;

pub struct VerifyingKey {
    verifying_key: ark_groth16::VerifyingKey<Bn254>,
}

impl VerifyingKey {
    pub fn from_compressed_bytes(bytes: &[u8]) -> Result<Self> {
        let verifying_key = ark_groth16::VerifyingKey::deserialize_compressed(bytes)?;

        Ok(Self { verifying_key })
    }

    pub fn into_core_type(self) -> Result<groth16::VerifyingKey> {
        let core_type = groth16::VerifyingKey::from_arkworks(self.verifying_key)?;

        Ok(core_type)
    }
}
