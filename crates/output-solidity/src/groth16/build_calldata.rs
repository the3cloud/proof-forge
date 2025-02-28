use anyhow::Result;
use proof_forge_core::groth16;

pub fn build_calldata(
    proof: &groth16::Proof,
    public_input: &groth16::PublicOutputs,
) -> Result<Vec<u8>> {
    let mut res = Vec::with_capacity(32 * 8 + 32 * public_input.pub_signals.len());

    res.extend_from_slice(&proof.a.x.to_be_bytes_vec());
    res.extend_from_slice(&proof.a.y.to_be_bytes_vec());
    res.extend_from_slice(&proof.b.x0.to_be_bytes_vec());
    res.extend_from_slice(&proof.b.x1.to_be_bytes_vec());
    res.extend_from_slice(&proof.b.y0.to_be_bytes_vec());
    res.extend_from_slice(&proof.b.y1.to_be_bytes_vec());
    res.extend_from_slice(&proof.c.x.to_be_bytes_vec());
    res.extend_from_slice(&proof.c.y.to_be_bytes_vec());

    for signal in public_input.pub_signals.iter() {
        res.extend_from_slice(&signal.to_be_bytes_vec());
    }

    Ok(res)
}
