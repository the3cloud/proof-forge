#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum Target {
    EVM,
    Solana,
    Ton,
    Sui,
    Cairo,
}

#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum TargetFormat {
    /// Only works for EVM target.
    Foundry,
    Javascript,
}

impl TargetFormat {
    pub fn to_str(&self) -> &str {
        match self {
            TargetFormat::Foundry => "foundry",
            TargetFormat::Javascript => "javascript",
        }
    }
}
