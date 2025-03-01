use std::fs;

use anyhow::Result;
use proof_forge_core::{Triple, ZKProofAlgorithm, ZKProofCurve, ZKProofImplementation};

use crate::forge;

use super::target::Target;

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[clap(short, long)]
    input_triple: String,

    #[clap(long)]
    verifying_key_path: Option<String>,

    #[clap(long)]
    proof_path: Option<String>,

    #[clap(long)]
    public_input_path: Option<String>,

    #[clap(long)]
    target: Target,
}

impl Args {
    pub fn run(self) -> Result<()> {
        let triple = Triple::new(&self.input_triple)?;

        let verifying_key = match self.verifying_key_path {
            Some(path) => Some(fs::read_to_string(&path)?),
            None => None,
        };

        let proof = match self.proof_path {
            Some(path) => Some(fs::read_to_string(&path)?),
            None => None,
        };

        let public_input = match self.public_input_path {
            Some(path) => Some(fs::read_to_string(&path)?),
            None => None,
        };

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
                forge::groth16_snarkjs_bn254_evm::build(verifying_key, proof, public_input)?;
            }
            (
                ZKProofAlgorithm::Groth16,
                ZKProofImplementation::Snarkjs,
                ZKProofCurve::BN254,
                Target::Sui,
            ) => {
                forge::groth16_snarkjs_bn254_sui::build(verifying_key, proof, public_input)?;
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported target: {:?}", self.target));
            }
        }

        println!("Forge Done");

        Ok(())
    }
}
