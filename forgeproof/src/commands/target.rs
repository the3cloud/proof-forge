#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum Target {
    EVM,
    Solana,
    Ton,
    Sui,
    Cairo,
}
