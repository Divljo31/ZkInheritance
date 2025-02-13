// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Groth16Verifier} from "./Verifier.sol";

contract Testament {
    struct Heir {
        address heirAddress;
        uint256 percentage;
        string documentHash;
    }

    struct TestamentData {
        address owner;
        address lawyer;
        bool isActive;
        Heir[] heirs;
    }

    mapping(uint256 => bool) private nullifiers;

    event ProofVerified(bool indexed result);
    event NullifierRegistered(uint256 nullifier);
    event DebugProof(
        uint[2] _pA,
        uint[2][2] _pB,
        uint[2] _pC,
        uint[3] _pubSignals
    );

    mapping(uint256 => TestamentData) public testaments;
    uint256 public testamentCount;

    event TestamentCreated(
        uint256 indexed testamentId,
        address owner,
        address lawyer
    );
    event TestamentActivated(uint256 indexed testamentId);
    event InheritanceClaimed(
        uint256 indexed testamentId,
        address heir,
        uint256 amount
    );

    constructor() {}

    function createTestament(address _lawyer) external returns (uint256) {
        testamentCount++;
        TestamentData storage newTestament = testaments[testamentCount];
        newTestament.owner = msg.sender;
        newTestament.lawyer = _lawyer;
        newTestament.isActive = false;

        emit TestamentCreated(testamentCount, msg.sender, _lawyer);
        return testamentCount;
    }

    function addHeir(
        uint256 _testamentId,
        address _heir,
        uint256 _percentage,
        string memory _documentHash
    ) external {
        require(
            msg.sender == testaments[_testamentId].owner,
            "Only owner can add heirs"
        );
        testaments[_testamentId].heirs.push(
            Heir(_heir, _percentage, _documentHash)
        );
    }

    function getHeirs(
        uint256 _testamentId
    ) external view returns (Heir[] memory) {
        return testaments[_testamentId].heirs;
    }

    function activateTestament(uint256 _testamentId) external {
        require(
            msg.sender == testaments[_testamentId].lawyer,
            "Only assigned lawyer can activate the testament"
        );
        testaments[_testamentId].isActive = true;
        emit TestamentActivated(_testamentId);
    }

    function viewInheritance(
        uint256 _testamentId
    ) external view returns (uint256, string memory) {
        for (uint256 i = 0; i < testaments[_testamentId].heirs.length; i++) {
            if (testaments[_testamentId].heirs[i].heirAddress == msg.sender) {
                return (
                    testaments[_testamentId].heirs[i].percentage,
                    testaments[_testamentId].heirs[i].documentHash
                );
            }
        }
        revert("You are not listed as an heir");
    }

    // Inheritance is activate => do transaction
    function claimInheritance(
        uint256 _testamentId,
        uint[2] calldata _pA,
        uint[2][2] calldata _pB,
        uint[2] calldata _pC,
        uint[3] calldata _pubSignals
    ) public payable {
        emit DebugProof(_pA, _pB, _pC, _pubSignals);
        uint256 nullifier = _pubSignals[0];

        require(!nullifiers[nullifier], "Error: Nullifier already exists!");
        nullifiers[nullifier] = true;
        emit NullifierRegistered(nullifier);

        Groth16Verifier verifier = new Groth16Verifier();
        bool result = verifier.verifyProof(_pA, _pB, _pC, _pubSignals);
        emit ProofVerified(result);

        require(testaments[_testamentId].isActive, "Testament is not active");

        for (uint256 i = 0; i < testaments[_testamentId].heirs.length; i++) {
            if (testaments[_testamentId].heirs[i].heirAddress == msg.sender) {
                uint256 amount = testaments[_testamentId].heirs[i].percentage;
                emit InheritanceClaimed(_testamentId, msg.sender, amount);
                return;
            }
        }
        revert("You are not listed as an heir");
    }
}