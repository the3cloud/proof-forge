use anyhow::Result;
use proof_forge_core::{Triple, ZKProofAlgorithm, ZKProofImplementation};

use super::target::Target;

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[clap(short, long)]
    input_triple: String,

    #[clap(short, long)]
    verifying_key_path: String,

    #[clap(short, long)]
    target: Target,

    output_path: String,
}

impl Args {
    pub fn run(self) -> Result<()> {
        let triple = Triple::new(&self.input_triple)?;
        let verifying_key = std::fs::read_to_string(&self.verifying_key_path)?;

        match (&triple.algorithm, &triple.implementation, &self.target) {
            (ZKProofAlgorithm::Groth16, ZKProofImplementation::Snarkjs, Target::EVM) => {
                let vk =
                    proof_forge_input_snarkjs::groth16::VerificationKey::from_str(&verifying_key)?;

                let vk = vk.into_core_type()?;

                let sol = proof_forge_output_solidity::groth16::build_verifier(&vk)?;

                std::fs::write(self.output_path, sol)?;

                Ok(())
            }
            _ => Err(anyhow::anyhow!(
                "Unsupported implementation: {:?}",
                triple.implementation
            )),
        }
    }
}
