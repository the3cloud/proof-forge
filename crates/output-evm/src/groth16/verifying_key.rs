use anyhow::Result;
use proof_forge_core::groth16;
use std::fmt::Write;
pub fn build_evm_verifying_key(vk: &groth16::VerifyingKey, format: &str) -> Result<String> {
    let mut v = String::new();

    if format == "foundry" {
        write!(v, "[")?;
        write!(v, "{},{},", vk.alpha.x, vk.alpha.y)?;
        write!(v, "{},{},", vk.beta.x0, vk.beta.x1)?;
        write!(v, "{},{},", vk.beta.y0, vk.beta.y1)?;
        write!(v, "{},{}", vk.gamma.x0, vk.gamma.x1)?;
        write!(v, "{},{}", vk.gamma.y0, vk.gamma.y1)?;
        write!(v, "{},{},", vk.delta.x0, vk.delta.x1)?;
        write!(v, "{},{}", vk.delta.y0, vk.delta.y1)?;
        for ic in &vk.ic {
            write!(v, "{},{}", ic.x, ic.y)?;
        }

        write!(v, "]")?;
    }

    Ok(v)
}
