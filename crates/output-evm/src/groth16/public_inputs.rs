use anyhow::Result;
use proof_forge_core::groth16;

pub fn build_evm_public_inputs(public_inputs: &groth16::PublicInputs) -> Result<Vec<u8>> {
    let mut v = Vec::new();

    for pub_signal in &public_inputs.pub_signals {
        v.extend_from_slice(&pub_signal.to_be_bytes::<32>());
    }

    Ok(v)
}
