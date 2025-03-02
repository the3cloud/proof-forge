use anyhow::Result;
use proof_forge_core::groth16;
use std::fmt::Write;

pub fn build_evm_public_inputs(
    public_inputs: &groth16::PublicInputs,
    format: &str,
) -> Result<String> {
    let mut v = String::new();

    if format == "foundry" {
        write!(v, "[")?;

        let mut it = public_inputs.pub_signals.iter();

        if let Some(pub_signal) = it.next() {
            write!(v, "{}", pub_signal)?;
        }

        for pub_signal in it {
            write!(v, ",{}", pub_signal)?;
        }
        write!(v, "]")?;
    }

    Ok(v)
}
