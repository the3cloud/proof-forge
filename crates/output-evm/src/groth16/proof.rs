use anyhow::Result;
use proof_forge_core::groth16::Proof;
use std::fmt::Write;

pub fn build_evm_proof(proof: &Proof, format: &str) -> Result<String> {
    let mut v = String::new();

    if format == "foundry" {
        write!(v, "[")?;
        write!(v, "{},{},", proof.a.x, proof.a.y)?;
        write!(v, "{},{},", proof.b.x0, proof.b.x1)?;
        write!(v, "{},{},", proof.b.y0, proof.b.y1)?;
        write!(v, "{},{}", proof.c.x, proof.c.y)?;
        write!(v, "]")?;
    }

    Ok(v)
}
