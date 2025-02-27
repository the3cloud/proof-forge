use anyhow::Result;
use proof_forge_core::groth16::VerificationKey;

use std::fmt::Write;

pub fn build_validate(vkey: &VerificationKey) -> Result<String> {
    let mut s = String::new();

    for i in 0..vkey.ic.len() {
        writeln!(s, "checkField(calldataload(add(_pubSignals, {})))", 32 * i)?;
    }

    Ok(s)
}
