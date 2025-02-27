# Proof Forge Contracts

> Unified Verifier for difference zk system.

## Features

- Provide generic verifier contract for all supported chain.
- Export verifier contract for special circuit.
- Widely supports multiple proof system implementations.

## Supported

### zkProof Implementation

- [ ] snarkjs
- [ ] arkworks
- [ ] gnark
- [ ] zkvm
  - [ ] risc0
  - [ ] sp1

### Support Chain

- EVM
- [ ] Solana
- [ ] Ton
- [ ] Move
- [ ] Cairo

## Usage

### Export Verifier Contract

### Forge VerifyingKey / Proof / PublicInput

You can forge output of zkproof implementation to used them in generic verifier contract.

## Proof Triple

The `Proof Triple` is a string to identify the proof type. It have the following format:

```
<algorithm>-<implementation>-<curve>:<version>
```


