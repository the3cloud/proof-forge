use std::fs;

use anyhow::Result;
use proof_forge_core::{Triple, ZKProofAlgorithm, ZKProofImplementation};

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

        match (&triple.algorithm, &triple.implementation, &self.target) {
            (ZKProofAlgorithm::Groth16, ZKProofImplementation::Snarkjs, Target::EVM) => {
                if let Some(verifying_key) = verifying_key {
                    let verifying_key =
                        proof_forge_input_snarkjs::groth16::VerificationKey::from_str(
                            &verifying_key,
                        )?;

                    let vk = verifying_key.into_core_type()?;
                    let vk = proof_forge_output_evm::groth16::build_evm_verifying_key(&vk)?;

                    println!("Verifying key: 0x{}\n", hex::encode(vk));
                }

                if let Some(proof) = proof {
                    let proof = proof_forge_input_snarkjs::groth16::Proof::from_str(&proof)?;

                    let proof = proof.into_core_type()?;
                    let proof = proof_forge_output_evm::groth16::build_evm_proof(&proof)?;

                    println!("Proof: 0x{}\n", hex::encode(proof));
                }

                if let Some(public_input) = public_input {
                    let public_input =
                        proof_forge_input_snarkjs::groth16::PublicInputs::from_str(&public_input)?;

                    let public_input = public_input.into_core_type()?;
                    let public_input =
                        proof_forge_output_evm::groth16::build_evm_public_inputs(&public_input)?;

                    println!("Public input: 0x{}\n", hex::encode(public_input));
                }
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported target: {:?}", self.target));
            }
        }

        println!("Forge Done");

        Ok(())
    }
}
