use anyhow::Result;
use ark_serialize::CanonicalSerialize;
use proof_forge_core::groth16;

pub fn build_sui_verifying_key(vk: &groth16::VerifyingKey) -> Result<Vec<u8>> {
    let mut v = Vec::new();

    let vk = vk.to_arkworks();
    vk.serialize_compressed(&mut v)?;

    Ok(v)
}

pub fn build_sui_proof(proof: &groth16::Proof) -> Result<Vec<u8>> {
    let mut v = Vec::new();

    let proof = proof.to_arkworks();
    proof.serialize_compressed(&mut v)?;

    Ok(v)
}

pub fn build_sui_public_inputs(public_inputs: &groth16::PublicInputs) -> Result<Vec<u8>> {
    let mut v = Vec::new();

    let public_inputs = public_inputs.to_arkworks();
    for signal in public_inputs.iter() {
        signal.serialize_compressed(&mut v)?;
    }

    Ok(v)
}
