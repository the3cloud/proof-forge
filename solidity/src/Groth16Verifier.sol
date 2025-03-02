// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.7.0 <0.9.0;

contract Groth16Verifier {
    // Scalar field size
    uint256 constant r =
        21888242871839275222246405745257275088548364400416034343698204186575808495617;
    // Base field size
    uint256 constant q =
        21888242871839275222246405745257275088696311157297823662689037894645226208583;

    // Memory data
    uint16 constant pVk = 0;
    uint16 constant pPairing = 128;

    uint16 constant pLastMem = 896;

    function verifyProof(
        uint256[] calldata _verifyingKey,
        uint256[8] calldata _proof,
        uint256[] calldata _pubSignals
    ) public view returns (bool) {
        require(
            _verifyingKey.length >= 16,
            "verifying key length must be >= 16"
        );
        require(
            _pubSignals.length * 2 + 16 == _verifyingKey.length,
            "invalid pubSignals length"
        );

        assembly {
            function checkField(v) {
                if iszero(lt(v, r)) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            // G1 function to multiply a G1 value(x,y) to value in an address
            function g1_mulAccC(pR, x, y, s) {
                let success
                let mIn := mload(0x40)
                mstore(mIn, x)
                mstore(add(mIn, 32), y)
                mstore(add(mIn, 64), s)

                success := staticcall(sub(gas(), 2000), 7, mIn, 96, mIn, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }

                mstore(add(mIn, 64), mload(pR))
                mstore(add(mIn, 96), mload(add(pR, 32)))

                success := staticcall(sub(gas(), 2000), 6, mIn, 128, pR, 64)

                if iszero(success) {
                    mstore(0, 0)
                    return(0, 0x20)
                }
            }

            function checkPairing(
                verifyingKey,
                proof,
                pubSignals,
                pubSignalsLength,
                pMem
            ) -> isOk {
                let _pPairing := add(pMem, pPairing)
                let _pVk := add(pMem, pVk)

                mstore(_pVk, calldataload(add(verifyingKey, 448)))
                mstore(add(_pVk, 32), calldataload(add(verifyingKey, 480)))

                // Compute the linear combination vk_x
                for {
                    let i := 0
                } lt(i, pubSignalsLength) {
                    i := add(i, 1)
                } {
                    g1_mulAccC(
                        _pVk,
                        calldataload(add(add(verifyingKey, 512), mul(i, 64))),
                        calldataload(add(add(verifyingKey, 544), mul(i, 64))),
                        calldataload(add(pubSignals, mul(i, 32)))
                    )
                }

                // -A
                mstore(_pPairing, calldataload(proof))
                mstore(
                    add(_pPairing, 32),
                    mod(sub(q, calldataload(add(proof, 32))), q)
                )

                // B
                mstore(add(_pPairing, 64), calldataload(add(proof, 64)))
                mstore(add(_pPairing, 96), calldataload(add(proof, 96)))
                mstore(add(_pPairing, 128), calldataload(add(proof, 128)))
                mstore(add(_pPairing, 160), calldataload(add(proof, 160)))

                // alpha1
                mstore(add(_pPairing, 192), calldataload(add(verifyingKey, 0)))
                mstore(add(_pPairing, 224), calldataload(add(verifyingKey, 32)))

                // beta2
                mstore(add(_pPairing, 256), calldataload(add(verifyingKey, 64)))
                mstore(add(_pPairing, 288), calldataload(add(verifyingKey, 96)))
                mstore(
                    add(_pPairing, 320),
                    calldataload(add(verifyingKey, 128))
                )
                mstore(
                    add(_pPairing, 352),
                    calldataload(add(verifyingKey, 160))
                )

                // vk_x
                mstore(add(_pPairing, 384), mload(add(pMem, pVk)))
                mstore(add(_pPairing, 416), mload(add(pMem, add(pVk, 32))))

                // gamma2
                mstore(
                    add(_pPairing, 448),
                    calldataload(add(verifyingKey, 192))
                )
                mstore(
                    add(_pPairing, 480),
                    calldataload(add(verifyingKey, 224))
                )
                mstore(
                    add(_pPairing, 512),
                    calldataload(add(verifyingKey, 256))
                )
                mstore(
                    add(_pPairing, 544),
                    calldataload(add(verifyingKey, 288))
                )

                // C
                mstore(add(_pPairing, 576), calldataload(add(proof, 192)))
                mstore(add(_pPairing, 608), calldataload(add(proof, 224)))

                // delta2
                mstore(
                    add(_pPairing, 640),
                    calldataload(add(verifyingKey, 320))
                )
                mstore(
                    add(_pPairing, 672),
                    calldataload(add(verifyingKey, 352))
                )
                mstore(
                    add(_pPairing, 704),
                    calldataload(add(verifyingKey, 384))
                )
                mstore(
                    add(_pPairing, 736),
                    calldataload(add(verifyingKey, 416))
                )

                let success := staticcall(
                    sub(gas(), 2000),
                    8,
                    _pPairing,
                    768,
                    _pPairing,
                    0x20
                )

                isOk := and(success, mload(_pPairing))
            }

            let pMem := mload(0x40)
            mstore(0x40, add(pMem, pLastMem))

            // Validate that all evaluations ∈ F
            let pubSignalsLength := _pubSignals.length

            for {
                let i := 0
            } lt(i, pubSignalsLength) {
                i := add(i, 1)
            } {
                checkField(calldataload(add(_pubSignals.offset, mul(i, 32))))
            }

            // Validate all evaluations
            let isValid := checkPairing(
                _verifyingKey.offset,
                _proof,
                _pubSignals.offset,
                pubSignalsLength,
                pMem
            )

            mstore(0, isValid)
            return(0, 0x20)
        }
    }
}
