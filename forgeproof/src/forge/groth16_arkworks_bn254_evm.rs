use anyhow::Result;

use crate::commands::target::TargetFormat;

pub fn build(
    verifying_key: Option<String>,
    proof: Option<String>,
    public_input: Option<String>,
    target_format: TargetFormat,
) -> Result<()> {
    if let Some(verifying_key) = verifying_key {
        let verifying_key = verifying_key.trim();

        let verifying_key =
            proof_forge_input_arkworks::groth16::VerifyingKey::from_compressed_bytes(
                &hex::decode(&verifying_key)?,
            )?;

        let vk = verifying_key.into_core_type()?;
        let vk =
            proof_forge_output_evm::groth16::build_evm_verifying_key(&vk, target_format.to_str())?;

        println!("Verifying key: {}\n", vk);
    }

    if let Some(proof) = proof {
        let proof = proof.trim();

        let proof = proof_forge_input_arkworks::groth16::Proof::from_compressed_bytes(
            &hex::decode(&proof)?,
        )?;

        let proof = proof.into_core_type()?;
        let proof =
            proof_forge_output_evm::groth16::build_evm_proof(&proof, target_format.to_str())?;

        println!("Proof: {}\n", proof);
    }

    if let Some(public_input) = public_input {
        let public_input = public_input.trim();

        let public_input = proof_forge_input_arkworks::groth16::PublicInputs::from_bytes(
            &hex::decode(&public_input)?,
        )?;

        let public_input = public_input.into_core_type()?;
        let public_input = proof_forge_output_evm::groth16::build_evm_public_inputs(
            &public_input,
            target_format.to_str(),
        )?;

        println!("Public input: {}\n", public_input);
    }

    Ok(())
}
