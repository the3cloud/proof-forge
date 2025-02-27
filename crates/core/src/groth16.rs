use crate::{G1Point, G2Point};
use alloy_primitives::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub a: G1Point,
    pub b: G2Point,
    pub c: G1Point,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationKey {
    pub alpha: G1Point,
    pub beta: G2Point,
    pub gamma: G2Point,
    pub delta: G2Point,
    pub ic: Vec<G1Point>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicOutputs {
    pub pub_signals: Vec<U256>,
}
