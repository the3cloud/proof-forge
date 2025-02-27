use anyhow::Result;
use proof_forge_core::groth16::VerificationKey;

mod build_vkey;

mod build_linear;

mod build_validate;

pub fn build_verifier(vkey: &VerificationKey) -> Result<String> {
    let s = include_str!("verifier.sol.template");

    let s = s
        .replace("<%verification_key_data%>", &build_vkey::build_vkey(vkey)?)
        .replace(
            "<%linear_combination_vk_x%>",
            &build_linear::build_linear(vkey)?,
        )
        .replace(
            "<%validate_evaluations%>",
            &build_validate::build_validate(vkey)?,
        );

    Ok(s)
}
