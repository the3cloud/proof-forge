use anyhow::Result;
use proof_forge_core::groth16::VerifyingKey;

use std::fmt::Write;

pub fn build_linear(vkey: &VerifyingKey) -> Result<String> {
    let mut s = String::new();

    let ic = &vkey.ic;

    for i in 1..ic.len() {
        writeln!(
            s,
            "                g1_mulAccC(_pVk, IC{}x, IC{}y, calldataload(add(pubSignals, {})))",
            i,
            i,
            (i - 1) * 32
        )?;
    }

    Ok(s)
}
