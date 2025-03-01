use anyhow::Result;
use proof_forge_core::groth16;

pub fn build_evm_verifying_key(vk: &groth16::VerifyingKey) -> Result<Vec<u8>> {
    let mut v = Vec::new();

    v.extend_from_slice(&vk.alpha.x.to_be_bytes::<32>());
    v.extend_from_slice(&vk.alpha.y.to_be_bytes::<32>());

    v.extend_from_slice(&vk.beta.x0.to_be_bytes::<32>());
    v.extend_from_slice(&vk.beta.x1.to_be_bytes::<32>());
    v.extend_from_slice(&vk.beta.y0.to_be_bytes::<32>());
    v.extend_from_slice(&vk.beta.y1.to_be_bytes::<32>());

    v.extend_from_slice(&vk.gamma.x0.to_be_bytes::<32>());
    v.extend_from_slice(&vk.gamma.x1.to_be_bytes::<32>());
    v.extend_from_slice(&vk.gamma.y0.to_be_bytes::<32>());
    v.extend_from_slice(&vk.gamma.y1.to_be_bytes::<32>());

    v.extend_from_slice(&vk.delta.x0.to_be_bytes::<32>());
    v.extend_from_slice(&vk.delta.x1.to_be_bytes::<32>());
    v.extend_from_slice(&vk.delta.y0.to_be_bytes::<32>());
    v.extend_from_slice(&vk.delta.y1.to_be_bytes::<32>());

    for ic in &vk.ic {
        v.extend_from_slice(&ic.x.to_be_bytes::<32>());
        v.extend_from_slice(&ic.y.to_be_bytes::<32>());
    }

    Ok(v)
}
