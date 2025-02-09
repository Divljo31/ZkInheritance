// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

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

    mapping(uint256 => TestamentData) public testaments;
    uint256 public testamentCount;

    event TestamentCreated(uint256 indexed testamentId, address owner, address lawyer);
    event TestamentActivated(uint256 indexed testamentId);
    event InheritanceClaimed(uint256 indexed testamentId, address heir, uint256 amount);

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

    function addHeir(uint256 _testamentId, address _heir, uint256 _percentage, string memory _documentHash) external {
        require(msg.sender == testaments[_testamentId].owner, "Only owner can add heirs");
        testaments[_testamentId].heirs.push(Heir(_heir, _percentage, _documentHash));
    }

    function getHeirs(uint256 _testamentId) external view returns (Heir[] memory) {
    return testaments[_testamentId].heirs;
    }


    function activateTestament(uint256 _testamentId) external {
        require(msg.sender == testaments[_testamentId].lawyer, "Only assigned lawyer can activate the testament");
        testaments[_testamentId].isActive = true;
        emit TestamentActivated(_testamentId);
    }

    // omogućava nasledniku da vidi šta nasledjuje pre aktivacije
    function viewInheritance(uint256 _testamentId) external view returns (uint256, string memory) {
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

    //  naslednik može povući sredstva tek nakon aktivacije testamenta
    function claimInheritance(uint256 _testamentId) external {
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
