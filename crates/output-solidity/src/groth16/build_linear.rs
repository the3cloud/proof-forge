use anyhow::Result;
use proof_forge_core::groth16::VerificationKey;

use std::fmt::Write;

pub fn build_linear(vkey: &VerificationKey) -> Result<String> {
    let mut s = String::new();

    let ic = &vkey.ic;

    for i in 0..ic.len() {
        writeln!(
            s,
            "                g1_mulAccC(_pVk, IC{}x, IC{}y, calldataload(add(pubSignals, {})))",
            i,
            i,
            i * 32
        )?;
    }

    Ok(s)
}
