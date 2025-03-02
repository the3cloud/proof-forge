use anyhow::Result;

pub fn build(
    verifying_key: Option<String>,
    proof: Option<String>,
    public_input: Option<String>,
) -> Result<()> {
    if let Some(verifying_key) = verifying_key {
        let verifying_key = verifying_key.trim();

        let verifying_key =
            proof_forge_input_gnark::groth16::VerifyingKey::from_gnark_compressed_bytes(
                &hex::decode(&verifying_key)?,
            )?;

        let vk = verifying_key.into_core_type()?;
        let vk = proof_forge_output_sui::groth16::build_sui_verifying_key(&vk)?;

        println!("Verifying key: {}\n", hex::encode(vk));
    }

    if let Some(proof) = proof {
        let proof = proof.trim();

        let proof = proof_forge_input_gnark::groth16::Proof::from_gnark_compressed_bytes(
            &hex::decode(&proof)?,
        )?;

        let proof = proof.into_core_type()?;
        let proof = proof_forge_output_sui::groth16::build_sui_proof(&proof)?;

        println!("Proof: {}\n", hex::encode(proof));
    }

    if let Some(public_input) = public_input {
        let public_input = public_input.trim();

        let public_input =
            proof_forge_input_gnark::groth16::PublicInputs::from_gnark_compressed_bytes(
                &hex::decode(&public_input)?,
            )?;

        let public_input = public_input.into_core_type()?;
        let public_input = proof_forge_output_sui::groth16::build_sui_public_inputs(&public_input)?;

        println!("Public input: {}\n", hex::encode(public_input));
    }

    Ok(())
}
