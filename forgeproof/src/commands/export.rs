use anyhow::Result;
use proof_forge_core::{Triple, ZKProofAlgorithm, ZKProofImplementation};

#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum Target {
    EVM,
    Solana,
    Ton,
    Move,
    Cairo,
}

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
                let vk = proof_forge_input_snarkjs::handle_groth16_verifying_key(
                    triple,
                    &verifying_key,
                )?;

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
