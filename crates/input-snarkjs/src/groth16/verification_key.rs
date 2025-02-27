use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationKey {
    protocol: String,
    curve: String,
    #[serde(rename = "nPublic")]
    n_public: u64,

    vk_alpha_1: Vec<String>,
    vk_beta_2: Vec<Vec<String>>,
    vk_gamma_2: Vec<Vec<String>>,
    vk_delta_2: Vec<Vec<String>>,
    #[serde(rename = "IC")]
    ic: Vec<Vec<String>>,
}

impl VerificationKey {
    pub fn from_str(s: &str) -> Result<Self> {
        let vk: Self = serde_json::from_str(s)?;
        Ok(vk)
    }

    pub fn into_core_type(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_key() {
        let vk = VerificationKey::from_str(include_str!("../../testdata/verification_key.json"))
            .unwrap();

        println!("{:#?}", vk);
    }
}
