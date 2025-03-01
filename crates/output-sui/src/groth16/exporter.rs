use anyhow::Result;
use ark_serialize::CanonicalSerialize;
use proof_forge_core::groth16::VerifyingKey;

use std::fmt::Write;

pub fn exporter_verifier(verifying_key: VerifyingKey) -> Result<String> {
    let mut s = String::new();

    let vk = verifying_key.to_arkworks();

    let mut key_bytes = Vec::new();
    vk.serialize_compressed(&mut key_bytes)?;

    writeln!(
        s,
        "public fun verify_proof(proof: vector<u8>, public_inputs: vector<u8>): bool {{"
    )?;

    writeln!(s, "    use sui::groth16;")?;
    writeln!(s)?;
    writeln!(
        s,
        "    let pvk = sui::groth16::prepare_verifying_key(&groth16::bn254(), &x\"{}\");",
        hex::encode(key_bytes)
    )?;

    writeln!(
        s,
        "    let proof_points = groth16::proof_points_from_bytes(proof);"
    )?;
    writeln!(
        s,
        "    let public_inputs = groth16::public_proof_inputs_from_bytes(public_inputs);"
    )?;
    writeln!(
        s,
        "    groth16::verify_groth16_proof(&groth16::bn254(), &pvk, &public_inputs, &proof_points)"
    )?;
    writeln!(s, "}}")?;

    Ok(s)
}
