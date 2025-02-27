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

## `Forge` Service

### Forge proof

POST `/api/v1/forge`

Request:

```json
{
    "source_triple": "groth16-bn254-risc0/1.4.0",
    "target_chain": "evm",
    "proof_infos": [
        {
            "proof": "",
            "output_digest": "",
            "program_id": ""
        }
    ]
}
```

Response:

```json
{
    "code": 0,
    "message": "success",
    "data": {
        "proof_id": "",
        "target_triple": "groth16-bn254-risc0/1.4.0",
        "selector": "0xaabbccdd"
    }
}
```

### Get forged result

GET `/api/v1/proof/<proof_id>`

Response:

```json
{
    "code": 0,
    "message": "success",
    "data": {
        "proof": "",
        "output_digest": "",
        "program_id": ""
    }
}
```
