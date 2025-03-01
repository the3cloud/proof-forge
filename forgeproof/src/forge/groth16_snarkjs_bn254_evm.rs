use anyhow::Result;

pub fn build(
    verifying_key: Option<String>,
    proof: Option<String>,
    public_input: Option<String>,
) -> Result<()> {
    if let Some(verifying_key) = verifying_key {
        let verifying_key =
            proof_forge_input_snarkjs::groth16::VerifyingKey::from_str(&verifying_key)?;

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
        let public_input = proof_forge_output_evm::groth16::build_evm_public_inputs(&public_input)?;

        println!("Public input: 0x{}\n", hex::encode(public_input));
    }

    Ok(())
}
