# Proof Forge Contracts

> Unified Verifier for difference zk system.

## Features

- Provide generic verifier contract for all supported chain.
- Export verifier contract for special circuit.
- Widely supports multiple proof system implementations.

## Supported

### zkProof Implementation

- snarkjs
- [ ] arkworks
- [ ] gnark
- [ ] zkvm
  - [ ] risc0
  - [ ] sp1

### Support Chain

- EVM
- [ ] Solana
- [ ] Move
- [ ] Cairo
- [ ] Ton

### Supported Algorithm

- Groth16
- [ ] PLONK

### Supported Curve

- BN254
- [ ] BLS12-381

## Usage

### Export Verifier Contract

If you want to export evm verifier contract from snarkjs, you can use the following command:

```bash
forgeproof export --input-triple groth16-snarkjs-bn254 --verifying-key-path <path-to-verifying-key> --target evm <path-to-output>
```

### Forge VerifyingKey / Proof / PublicInput

You can forge output of zkproof implementation to used them in generic verifier contract.

```bash
# forge verifying key
forgeproof forge --input-triple <path-to-proof> --verifying-key-path <path-to-verifying-key> <path-to-output>

# forge proof
forgeproof forge --input-triple <path-to-proof> --proof-path <path-to-proof> <path-to-output>

# forge public input
forgeproof forge --input-triple <path-to-proof> --public-input-path <path-to-public-input> <path-to-output>

```

## Proof Triple

The `Proof Triple` is a string to identify the proof type. It have the following format:

```
<algorithm>-<implementation>-<curve>:<version>
```


