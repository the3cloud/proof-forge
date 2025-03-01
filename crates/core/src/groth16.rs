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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationKey {
    pub curve: ZKProofCurve,

    pub alpha: G1Point,
    pub beta: G2Point,
    pub gamma: G2Point,
    pub delta: G2Point,
    pub ic: Vec<G1Point>,
}

impl VerificationKey {
    pub fn n_public(&self) -> u64 {
        self.ic.len() as u64 - 1
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicInputs {
    pub pub_signals: Vec<U256>,
}
