use anyhow::Result;
use proof_forge_core::{groth16, Triple, ZKProofAlgorithm, ZKProofCurve};

use crate::groth16::VerificationKey;

pub fn handle_groth16_verifying_key(triple: Triple, s: &str) -> Result<groth16::VerificationKey> {
    if triple.algorithm != ZKProofAlgorithm::Groth16 {
        return Err(anyhow::anyhow!(
            "Unsupported algorithm: {:?}",
            triple.algorithm
        ));
    }

    let vk: VerificationKey = serde_json::from_str(s)?;

    if vk.protocol != "groth16" {
        return Err(anyhow::anyhow!("Unsupported protocol: {:?}", vk.protocol));
    }

    match triple.curve {
        ZKProofCurve::BN254 => {
            if vk.curve != "bn254" && vk.curve != "bn128" {
                return Err(anyhow::anyhow!("Unsupported curve: {:?}", vk.curve));
            }
        }
        _ => {
            return Err(anyhow::anyhow!("Unsupported curve: {:?}", vk.curve));
        }
    }

    Ok(vk.into_core_type()?)
}
