use anyhow::{anyhow, Result};
use proof_forge_core::groth16;

use crate::curve::scalar::Scalar;

#[derive(Debug)]
pub struct PublicInputs {
    signals: Vec<Scalar>,
}

impl PublicInputs {
    pub fn from_gnark_compressed_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 4 {
            return Err(anyhow!("Invalid public inputs length"));
        }

        let signals_len = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);

        if bytes.len() < 12 + signals_len as usize * 32 {
            return Err(anyhow!("Invalid public inputs length"));
        }

        let mut signals = Vec::new();

        for i in 0..signals_len {
            let begin = 12 + i as usize * 32;
            let end = begin + 32;

            let signal = Scalar::from_gnark_compressed_bytes(&bytes[begin..end])?;
            signals.push(signal);
        }

        Ok(Self { signals })
    }

    pub fn into_core_type(self) -> Result<groth16::PublicInputs> {
        let signals = self
            .signals
            .into_iter()
            .map(|signal| signal.into_core_type())
            .collect::<Result<Vec<_>>>()?;

        Ok(groth16::PublicInputs {
            pub_signals: signals,
        })
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::U256;

    use super::*;

    #[test]
    fn test_from_gnark_compressed_bytes() {
        let bytes = hex::decode("0000000100000000000000010000000000000000000000000000000000000000000000000000000000000021").unwrap();
        let public_inputs = PublicInputs::from_gnark_compressed_bytes(&bytes).unwrap();

        let core_type = public_inputs.into_core_type().unwrap();

        assert_eq!(core_type.pub_signals.len(), 1);
        assert_eq!(core_type.pub_signals[0], U256::from(33));
    }
}
