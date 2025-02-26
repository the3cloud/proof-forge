# Proof Forge Contracts

> Unified Verifier for difference zk system.

## Features

- A set of contract in multi chain to verify all supported platform.
- A service to `forge` your proof into a united contracts.

## Platform

Support Prover System:

- Risc0
- Sp1

Support Chain:

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
