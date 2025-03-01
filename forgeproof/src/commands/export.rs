use anyhow::Result;
use proof_forge_core::{Triple, ZKProofAlgorithm, ZKProofCurve, ZKProofImplementation};

use crate::exporter;

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

        match (
            &triple.algorithm,
            &triple.implementation,
            &triple.curve,
            &self.target,
        ) {
            (
                ZKProofAlgorithm::Groth16,
                ZKProofImplementation::Snarkjs,
                ZKProofCurve::BN254,
                Target::EVM,
            ) => {
                exporter::groth16_snarkjs_bn254_evm::export(&verifying_key, &self.output_path)?;

                Ok(())
            }
            (
                ZKProofAlgorithm::Groth16,
                ZKProofImplementation::Snarkjs,
                ZKProofCurve::BN254,
                Target::Sui,
            ) => {
                exporter::groth16_snarkjs_bn254_sui::export(&verifying_key, &self.output_path)?;

                Ok(())
            }
            _ => Err(anyhow::anyhow!(
                "Unsupported implementation: {:?}",
                triple.implementation
            )),
        }
    }
}
