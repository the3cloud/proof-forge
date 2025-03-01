use anyhow::Result;
use proof_forge_core::groth16::VerifyingKey;

mod build_linear;
mod build_validate;
mod build_vkey;

pub fn build_verifier(vkey: &VerifyingKey) -> Result<String> {
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
        )
        .replace("<%n_public%>", &format!("{}", vkey.n_public()));

    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_verifier() {
        let vkey: VerifyingKey =
            serde_json::from_str(include_str!("../../../testdata/verifying_key.json")).unwrap();

        let s = build_verifier(&vkey).unwrap();

        println!("{}", s);
    }
}
