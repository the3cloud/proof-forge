use anyhow::{Ok, Result};
use proof_forge_core::{groth16, G1Point, G2Point};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VerificationKey {
    pub protocol: String,
    pub curve: String,
    #[serde(rename = "nPublic")]
    pub n_public: u64,

    pub vk_alpha_1: Vec<String>,
    pub vk_beta_2: Vec<Vec<String>>,
    pub vk_gamma_2: Vec<Vec<String>>,
    pub vk_delta_2: Vec<Vec<String>>,
    #[serde(rename = "IC")]
    pub ics: Vec<Vec<String>>,
}

impl VerificationKey {
    pub fn from_str(s: &str) -> Result<Self> {
        let vk: Self = serde_json::from_str(s)?;
        Ok(vk)
    }

    pub fn into_core_type(self) -> Result<groth16::VerificationKey> {
        let alpha = G1Point::from_oct_str(&self.vk_alpha_1[0], &self.vk_alpha_1[1])?;
        let beta = G2Point::from_oct_str(
            &self.vk_beta_2[0][0],
            &self.vk_beta_2[0][1],
            &self.vk_beta_2[1][0],
            &self.vk_beta_2[1][1],
        )?;
        let gamma = G2Point::from_oct_str(
            &self.vk_gamma_2[0][0],
            &self.vk_gamma_2[0][1],
            &self.vk_gamma_2[1][0],
            &self.vk_gamma_2[1][1],
        )?;
        let delta = G2Point::from_oct_str(
            &self.vk_delta_2[0][0],
            &self.vk_delta_2[0][1],
            &self.vk_delta_2[1][0],
            &self.vk_delta_2[1][1],
        )?;

        let mut ics = Vec::new();

        for i in 0..self.ics.len() {
            let ic = G1Point::from_oct_str(&self.ics[i][0], &self.ics[i][1])?;
            ics.push(ic);
        }

        Ok(groth16::VerificationKey {
            alpha,
            beta,
            gamma,
            delta,
            ic: ics,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verification_key() {
        let vk = VerificationKey::from_str(include_str!("../../testdata/verification_key.json"))
            .unwrap();

        let core_vk = vk.into_core_type().unwrap();

        let s = serde_json::to_string(&core_vk).unwrap();
        println!("{}", s);
    }
}
