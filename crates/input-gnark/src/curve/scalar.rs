use alloy_primitives::U256;
use anyhow::{anyhow, Result};
use ark_bn254::Fr;
use ark_ff::{BigInteger, PrimeField};

#[derive(Debug)]
pub struct Scalar {
    value: Fr,
}

impl Scalar {
    pub fn from_gnark_compressed_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != 32 {
            return Err(anyhow!("Invalid scalar length"));
        }

        let value = Fr::from_be_bytes_mod_order(bytes);
        Ok(Self { value })
    }

    pub fn into_core_type(self) -> Result<U256> {
        let bytes = self.value.into_bigint().to_bytes_be();
        let bytes = U256::from_be_slice(&bytes);

        Ok(bytes)
    }
}
