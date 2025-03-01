use anyhow::Result;
use proof_forge_core::{groth16, G1Point, G2Point, ZKProofCurve};
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

        if proof.protocol != "groth16" {
            return Err(anyhow::anyhow!(
                "Invalid protocol, only groth16 is supported"
            ));
        }

        if proof.curve != "bn254" && proof.curve != "bn128" && proof.curve != "bls12-381" {
            return Err(anyhow::anyhow!(
                "Invalid curve, only bn254, bn128 and bls12-381 are supported"
            ));
        }

        Ok(proof)
    }

    pub fn into_core_type(self) -> Result<groth16::Proof> {
        let curve = match self.curve.as_str() {
            "bn254" => ZKProofCurve::BN254,
            "bn128" => ZKProofCurve::BN254,
            "bls12-381" => ZKProofCurve::BLS12_381,
            _ => return Err(anyhow::anyhow!("Invalid curve")),
        };

        let a = G1Point::from_oct_str(&self.pi_a[0], &self.pi_a[1])?;
        let b = G2Point::from_oct_str(
            &self.pi_b[0][1],
            &self.pi_b[0][0],
            &self.pi_b[1][1],
            &self.pi_b[1][0],
        )?;
        let c = G1Point::from_oct_str(&self.pi_c[0], &self.pi_c[1])?;

        Ok(groth16::Proof { curve, a, b, c })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof() {
        let proof = Proof::from_str(include_str!("../../testdata/proof.json")).unwrap();
        let core_proof = proof.into_core_type().unwrap();

        let s = serde_json::to_string(&core_proof).unwrap();
        println!("{}", s);
    }
}
