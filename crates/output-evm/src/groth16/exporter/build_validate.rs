use anyhow::Result;
use proof_forge_core::groth16::VerifyingKey;

use std::fmt::Write;

pub fn build_validate(vkey: &VerifyingKey) -> Result<String> {
    let mut s = String::new();

    for i in 0..vkey.ic.len() - 1 {
        writeln!(
            s,
            "            checkField(calldataload(add(_pubSignals, {})))",
            32 * i
        )?;
    }

    Ok(s)
}
