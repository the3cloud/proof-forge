use anyhow::Result;

pub fn export(verifying_key: &str, output_path: &str) -> Result<()> {
    let vk = proof_forge_input_snarkjs::groth16::VerifyingKey::from_str(&verifying_key)?;

    let vk = vk.into_core_type()?;

    let sol = proof_forge_output_evm::groth16::build_verifier(&vk)?;

    std::fs::write(output_path, sol)?;

    Ok(())
}
