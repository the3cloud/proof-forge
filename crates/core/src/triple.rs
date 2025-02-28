use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZKProofAlgorithm {
    Groth16,
    PLONK,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZKProofCurve {
    #[serde(rename = "bn254")]
    BN254,
    #[serde(rename = "bls12-381")]
    BLS12_381,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZKProofImplementation {
    Snarkjs,
    Arkworks,
    Gnark,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZKProofVersion {
    Latest,
    Specific(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Triple {
    pub algorithm: ZKProofAlgorithm,
    pub curve: ZKProofCurve,
    pub implementation: ZKProofImplementation,
    pub version: ZKProofVersion,
}

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^([a-zA-Z0-9_\-]+)-([a-zA-Z0-9_\-]+)-([a-zA-Z0-9_\-]+)(?::([a-zA-Z0-9_\-\.]+))?$")
        .unwrap()
});

impl Triple {
    pub fn new(s: &str) -> Result<Self> {
        let captures = REGEX.captures(s).ok_or(anyhow::anyhow!("Invalid triple"))?;

        let algorithm = captures
            .get(1)
            .ok_or(anyhow::anyhow!("Invalid algorithm"))?
            .as_str();

        let algorithm = match algorithm {
            "groth16" => ZKProofAlgorithm::Groth16,
            "plonk" => ZKProofAlgorithm::PLONK,
            _ => return Err(anyhow::anyhow!("Invalid algorithm")),
        };

        let implementation = captures
            .get(2)
            .ok_or(anyhow::anyhow!("Invalid implementation"))?
            .as_str();

        let implementation = match implementation {
            "snarkjs" => ZKProofImplementation::Snarkjs,
            "arkworks" => ZKProofImplementation::Arkworks,
            "gnark" => ZKProofImplementation::Gnark,
            _ => return Err(anyhow::anyhow!("Invalid implementation")),
        };

        let curve = captures
            .get(3)
            .ok_or(anyhow::anyhow!("Invalid curve"))?
            .as_str();

        let curve = match curve {
            "bn254" => ZKProofCurve::BN254,
            "bn128" => ZKProofCurve::BN254,
            "bls12_381" => ZKProofCurve::BLS12_381,
            _ => return Err(anyhow::anyhow!("Invalid curve")),
        };

        let version = if let Some(version) = captures.get(4) {
            let version = version.as_str();

            ZKProofVersion::Specific(version.into())
        } else {
            ZKProofVersion::Latest
        };

        Ok(Self {
            algorithm,
            implementation,
            curve,
            version,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triple_parse() {
        // Test basic triple without version
        let triple = Triple::new("groth16-snarkjs-bn254").unwrap();
        assert_eq!(triple.algorithm, ZKProofAlgorithm::Groth16);
        assert_eq!(triple.implementation, ZKProofImplementation::Snarkjs);
        assert_eq!(triple.curve, ZKProofCurve::BN254);
        assert_eq!(triple.version, ZKProofVersion::Latest);

        // Test triple with version
        let triple = Triple::new("plonk-arkworks-bls12_381:v1.0").unwrap();
        assert_eq!(triple.algorithm, ZKProofAlgorithm::PLONK);
        assert_eq!(triple.implementation, ZKProofImplementation::Arkworks);
        assert_eq!(triple.curve, ZKProofCurve::BLS12_381);
        assert_eq!(triple.version, ZKProofVersion::Specific("v1.0".to_string()));

        // Test invalid cases
        assert!(Triple::new("invalid").is_err());
        assert!(Triple::new("invalid-snarkjs-bn254").is_err());
        assert!(Triple::new("groth16-invalid-bn254").is_err());
        assert!(Triple::new("groth16-snarkjs-invalid").is_err());
    }
}
