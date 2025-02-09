const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Testament Contract", function () {
    let Testament, testament, owner, lawyer, heir1, heir2;

    beforeEach(async function () {
        [owner, lawyer, heir1, heir2] = await ethers.getSigners();

        Testament = await ethers.getContractFactory("Testament");
        testament = await Testament.deploy();
        await testament.waitForDeployment();

        console.log(`Testament deployed at: ${testament.target}`);
    });

    it("Treba da kreira testament sa advokatom", async function () {
        await testament.connect(owner).createTestament(lawyer.address);
        expect(await testament.testamentCount()).to.equal(1);
    });

    it("Treba da doda naslednika", async function () {
        await testament.connect(owner).createTestament(lawyer.address);
        await testament.connect(owner).addHeir(1, heir1.address, 50, "test-hash");

        const heirs = await testament.getHeirs(1);
        expect(heirs.length).to.equal(1);
    });

    it("Samo advokat može aktivirati testament", async function () {
        await testament.connect(owner).createTestament(lawyer.address);

        await expect(
            testament.connect(owner).activateTestament(1)
        ).to.be.revertedWith("Only assigned lawyer can activate the testament");

        await testament.connect(lawyer).activateTestament(1);
        const testData = await testament.testaments(1);
        expect(testData.isActive).to.equal(true);
    });

    it("Naslednik može da vidi svoje nasledstvo pre aktivacije", async function () {
        await testament.connect(owner).createTestament(lawyer.address);
        await testament.connect(owner).addHeir(1, heir1.address, 50, "test-hash");

        const [percentage, documentHash] = await testament.connect(heir1).viewInheritance(1);
        expect(percentage).to.equal(50);
        expect(documentHash).to.equal("test-hash");
    });

    it("Naslednik prima imovinu tek nakon aktivacije testamenta", async function () {
        await testament.connect(owner).createTestament(lawyer.address);
        await testament.connect(owner).addHeir(1, heir1.address, 50, "test-hash");
        await testament.connect(lawyer).activateTestament(1);

        await expect(
            testament.connect(heir1).claimInheritance(1)
        ).to.emit(testament, "InheritanceClaimed").withArgs(1, heir1.address, 50);
    });
});