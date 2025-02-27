use alloy_primitives::U256;
use anyhow::Result;
use proof_forge_core::groth16;

pub struct PublicOutputs {
    pub pub_signals: Vec<String>,
}

impl PublicOutputs {
    pub fn from_str(s: &str) -> Result<Self> {
        let pub_signals: Vec<String> = serde_json::from_str(s)?;

        Ok(Self { pub_signals })
    }

    pub fn into_core_type(self) -> Result<groth16::PublicOutputs> {
        let mut pub_signals = Vec::new();

        for signal in self.pub_signals {
            let signal: U256 = signal.parse()?;
            pub_signals.push(signal);
        }

        Ok(groth16::PublicOutputs { pub_signals })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_outputs() {
        let public_outputs =
            PublicOutputs::from_str(include_str!("../../testdata/public.json")).unwrap();

        let core_public_outputs = public_outputs.into_core_type().unwrap();

        println!("{:#?}", core_public_outputs);
    }
}
