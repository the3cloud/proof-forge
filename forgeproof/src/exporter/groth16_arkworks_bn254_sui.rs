use anyhow::Result;

pub fn export(verifying_key: &str, output_path: &str) -> Result<()> {
    let verifying_key = verifying_key.trim();

    let vk = proof_forge_input_arkworks::groth16::VerifyingKey::from_compressed_bytes(
        &hex::decode(verifying_key)?,
    )?;
    let vk = vk.into_core_type()?;

    log::debug!("vk: {:#?}", vk);

    let sol = proof_forge_output_evm::groth16::build_verifier(&vk)?;

    std::fs::write(output_path, sol)?;

    Ok(())
}
