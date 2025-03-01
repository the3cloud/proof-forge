use anyhow::Result;
use proof_forge_core::groth16::Proof;

pub fn build_evm_proof(proof: &Proof) -> Result<Vec<u8>> {
    let mut v = Vec::new();

    v.extend_from_slice(&proof.a.x.to_be_bytes::<32>());
    v.extend_from_slice(&proof.a.y.to_be_bytes::<32>());

    v.extend_from_slice(&proof.b.x0.to_be_bytes::<32>());
    v.extend_from_slice(&proof.b.x1.to_be_bytes::<32>());
    v.extend_from_slice(&proof.b.y0.to_be_bytes::<32>());
    v.extend_from_slice(&proof.b.y1.to_be_bytes::<32>());

    v.extend_from_slice(&proof.c.x.to_be_bytes::<32>());
    v.extend_from_slice(&proof.c.y.to_be_bytes::<32>());

    Ok(v)
}
