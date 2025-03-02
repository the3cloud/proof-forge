use anyhow::Result;
use ark_bn254::Fr;
use ark_ff::PrimeField;
use proof_forge_core::groth16;

pub struct PublicInputs {
    inputs: Vec<Fr>,
}

impl PublicInputs {
    pub fn from_arkworks(inputs: &[u8]) -> Result<Self> {
        let mut res = Vec::new();

        let inputs = inputs.chunks(32);

        for input in inputs {
            let input = Fr::from_be_bytes_mod_order(input);
            res.push(input);
        }

        Ok(Self { inputs: res })
    }

    pub fn into_core_type(self) -> Result<groth16::PublicInputs> {
        let core_type = groth16::PublicInputs::from_arkworks(self.inputs)?;

        Ok(core_type)
    }
}
