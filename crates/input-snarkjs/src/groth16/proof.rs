use anyhow::Result;
use proof_forge_core::{groth16, G1Point, G2Point};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub protocol: String,
    pub curve: String,
    pub pi_a: Vec<String>,
    pub pi_b: Vec<Vec<String>>,
    pub pi_c: Vec<String>,
}

impl Proof {
    pub fn from_str(s: &str) -> Result<Self> {
        let proof: Self = serde_json::from_str(s)?;
        Ok(proof)
    }

    pub fn into_core_type(self) -> Result<groth16::Proof> {
        let a = G1Point::from_oct_str(&self.pi_a[0], &self.pi_a[1])?;
        let b = G2Point::from_oct_str(
            &self.pi_b[0][0],
            &self.pi_b[0][1],
            &self.pi_b[1][0],
            &self.pi_b[1][1],
        )?;
        let c = G1Point::from_oct_str(&self.pi_c[0], &self.pi_c[1])?;

        Ok(groth16::Proof { a, b, c })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof() {
        let proof = Proof::from_str(include_str!("../../testdata/proof.json")).unwrap();
        let core_proof = proof.into_core_type().unwrap();

        println!("{:#?}", core_proof);
    }
}
