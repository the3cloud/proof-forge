use std::fs;

use anyhow::Result;
use proof_forge_core::{Triple, ZKProofAlgorithm, ZKProofImplementation};

use super::target::Target;

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[clap(long)]
    input_triple: String,

    #[clap(long)]
    proof_path: String,

    #[clap(long)]
    public_input_path: String,

    #[clap(long)]
    target: Target,
}

impl Args {
    pub fn run(self) -> Result<()> {
        let triple = Triple::new(&self.input_triple)?;

        let proof = fs::read_to_string(&self.proof_path)?;
        let public_input = fs::read_to_string(&self.public_input_path)?;

        match (&triple.algorithm, &triple.implementation, &self.target) {
            (ZKProofAlgorithm::Groth16, ZKProofImplementation::Snarkjs, Target::EVM) => {
                let proof = proof_forge_input_snarkjs::handle_groth16_proof(&triple, &proof)?;
                let public_input =
                    proof_forge_input_snarkjs::handle_groth16_public_outputs(&public_input)?;

                let calldata =
                    proof_forge_output_solidity::groth16::build_calldata(&proof, &public_input)?;
                println!("0x{}", hex::encode(calldata));
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported target: {:?}", self.target));
            }
        }

        Ok(())
    }
}
