use crate::{G1Point, G2Point, ZKProofCurve};
use alloy_primitives::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub curve: ZKProofCurve,

    pub a: G1Point,
    pub b: G2Point,
    pub c: G1Point,
}

impl Proof {
    #[cfg(feature = "arkworks")]
    pub fn to_arkworks(&self) -> ark_groth16::Proof<ark_bn254::Bn254> {
        let a = self.a.to_arkworks();
        let b = self.b.to_arkworks();
        let c = self.c.to_arkworks();

        ark_groth16::Proof { a, b, c }
    }

    #[cfg(feature = "arkworks")]
    pub fn from_arkworks(proof: ark_groth16::Proof<ark_bn254::Bn254>) -> anyhow::Result<Self> {
        let a = G1Point::from_arkworks(proof.a);
        let b = G2Point::from_arkworks(proof.b)?;
        let c = G1Point::from_arkworks(proof.c);

        Ok(Self {
            curve: ZKProofCurve::BN254,
            a,
            b,
            c,
        })
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyingKey {
    pub curve: ZKProofCurve,

    pub alpha: G1Point,
    pub beta: G2Point,
    pub gamma: G2Point,
    pub delta: G2Point,
    pub ic: Vec<G1Point>,
}

impl VerifyingKey {
    pub fn n_public(&self) -> u64 {
        self.ic.len() as u64 - 1
    }

    #[cfg(feature = "arkworks")]
    pub fn to_arkworks(&self) -> ark_groth16::VerifyingKey<ark_bn254::Bn254> {
        let alpha = self.alpha.to_arkworks();
        let beta = self.beta.to_arkworks();
        let gamma = self.gamma.to_arkworks();
        let delta = self.delta.to_arkworks();

        let mut ic = Vec::new();

        for point in self.ic.iter() {
            ic.push(point.to_arkworks());
        }

        ark_groth16::VerifyingKey {
            alpha_g1: alpha,
            beta_g2: beta,
            gamma_g2: gamma,
            delta_g2: delta,
            gamma_abc_g1: ic,
        }
    }

    #[cfg(feature = "arkworks")]
    pub fn from_arkworks(
        verifying_key: ark_groth16::VerifyingKey<ark_bn254::Bn254>,
    ) -> anyhow::Result<Self> {
        let alpha = G1Point::from_arkworks(verifying_key.alpha_g1);
        let beta = G2Point::from_arkworks(verifying_key.beta_g2)?;
        let gamma = G2Point::from_arkworks(verifying_key.gamma_g2)?;
        let delta = G2Point::from_arkworks(verifying_key.delta_g2)?;

        let mut ic = Vec::new();

        for point in verifying_key.gamma_abc_g1 {
            ic.push(G1Point::from_arkworks(point));
        }

        Ok(Self {
            curve: ZKProofCurve::BN254,
            alpha,
            beta,
            gamma,
            delta,
            ic,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicInputs {
    pub pub_signals: Vec<U256>,
}

impl PublicInputs {
    #[cfg(feature = "arkworks")]
    pub fn to_arkworks(&self) -> Vec<ark_bn254::Fr> {
        use ark_ff::PrimeField;

        let mut pub_signals = Vec::new();

        for signal in self.pub_signals.iter() {
            let bytes = signal.as_le_bytes();
            let field = ark_bn254::Fr::from_le_bytes_mod_order(&bytes);
            pub_signals.push(field);
        }

        pub_signals
    }

    #[cfg(feature = "arkworks")]
    pub fn from_arkworks(public_inputs: Vec<ark_bn254::Fr>) -> anyhow::Result<Self> {
        use ark_ff::{BigInteger, PrimeField};

        let mut pub_signals = Vec::new();

        for field in public_inputs.iter() {
            let bytes = field.into_bigint().to_bytes_be();
            let signal = U256::from_be_slice(&bytes);
            pub_signals.push(signal);
        }

        Ok(Self { pub_signals })
    }
}
