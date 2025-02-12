<script lang="ts">
  import { sha256 } from "js-sha256";
  import { groth16 } from "snarkjs";
  import { Library, zkVerifySession } from "zkverifyjs";
  
  let formData = {
    testamentId: "",
    passportId: "",
    name: "",
    walletAddress: "",
    hashes: "",
    privkey: "",
    root: ""
  };
  async function handleSubmit(event: SubmitEvent) {
    const session = await zkVerifySession
      .start()
      .Testnet()
      .withAccount(
        "0x1700010001016fd0108eda098428efe7ee7e76040c20a09abdfec0571e5868aeb65ab6ed001d76e5c9b2f105b2fa8f6ccb296e6cccffe13bd1e1240113a0b6aecaab21dab6170102f23f4911eb6eeea75a86c965f6553416a4c00f8144d22f921fff9681bc5f202e28f2e8e7905e8243da94018311ee8523cff1403b72fa71e922a2aed05ee17e04da744814cad9347145be9cd8d64ce561c21885b4c39f060f43b4437738aff12060db08614e87147f257b161cd5bc2503922195f6a7e6ae28460139cf8af448280102edf692d95cbdde46ddda5ef7d422436779445c5e66006a42761e1f12efde0018c212f3aeb785e49712e7a9353349aaf1255dfb31b7bf60723a480d9293938e19aa7dfa6601cce64c7bd3430c69e7d1e38f40cb8d8071ab4aeb6d8cdba55ec8125b9722d1dcdaac55f38eb37033314bbc95330c69ad999eec75f05f58d0890609010297dd1d066326315ebe3ab99f47e63c6adb846732a8234089ef2aadf4083ff507ec1892590e30ace0d37249aae6b47e27a03e2a10287e3d3b30141b8067e93c0793735a75026739f8e86450d2809b05a15c5dfd1a1a6fd0d6388b65e2baf6e911823a38904f2c7710a4e73e22aca4f867a3fe12c55c07e2bee487c1587837dd2b100101033ac57b01a029f891337b5a0baea8ae38d0bfa12a6ce86d9e5abb5e56a93204133f17b45821168592cc235d2b559272327f9f0b5505f3a9198ca617a817f82f01019e7cc705310fd61d7253f2cad22d4f6558258ab3f6614a679cfdddd672519106215af4c1e58fd29477326b6145dc917a3607b004cd7a13a4e1e5d7d24498cd27010108c3d96f1401034f365332f86bb57588f418739bbf1fcb0f331a5b726b8fe70037f5b8f0be8581c0a9ca971d4fc86aa335afb372b8666f01b0698f37fba167240101ba64f4ce78d10920d9ee741a2b99d7961e9d06100204d7f39eb4a19369b8e52a18e3641f6284a413f35fc9c10d7b02866615e5874824463db3a23558e9a8bc1100010197292bb260819ba1a84a2495039e04b6ce2ce9b7aad19f3a58e8998d5d0462241d875a13b9b122fb6eb1a9eb4c4590da65129f0342e064e7fc01286925100a1e0102a0455eefe7bf00c79593810186d892746bfdbef512a991f82abc35c13120b21e0332dffc6ef5b94ab902fe146b600e582a0c54a79f3873c081e351479230bf2d266a21a4ec11f513c4e5d5b1c357247854ad8263729e8a1eab3c05e60b274d0f2f653e9193829504dac35e67ff4beb307b6aa69efbf15d3fd5538f12d9758804010169590adf94d4a5f1237fb6d167d0f6b8f878896914e6cb6e5db398068629340c0f29e411649b06044713886a910e8efcb65f08207a21a08e0b0fed00664c33100c80cd8c9c24c01f0cab4e25bc4a62747bacb96aecd79a2c32508240be97b7eb731280102700000000000000000000000000000000000000000000000000000000000080b8bc6bf5f999703305f1d7cc961131c5ba66019fe12ed8c22edac3303599941100",
      );

    let pathElements: string[] = [];
    let pathIndex: string[] = [];
    for (let i = 0; i < formData.hashes.length; i++) {
      pathElements.push(formData.hashes[i]);
      pathIndex.push(formData.hashes[i].toString());
    }

    const circomData = {
      hashName: BigInt("0x" + sha256(formData.name)).toString(),
      pid: formData.passportId,
      privKey: formData.privkey,
      path_elements: pathElements,
      path_index: pathIndex,
      publicVar: "10000",
      root: formData.root,
    };

    const readOnlySession = await zkVerifySession.start().Testnet();

    const { events, transactionResult } = await session
      .verify()
      .groth16(Library.snarkjs, curve_bn128)
      .execute({
        proofData: {
          vk: "/Users/amar/Desktop/ZkInheritance/circuit/verification_key_zkv.json",
          proof: "/Users/amar/Desktop/ZkInheritance/circuit/proof_zkv.json",
          publicSignals:
            "/Users/amar/Desktop/ZkInheritance/circuit/public_zkv.json",
        },
      });

    events.on("ErrorEvent", (eventData) => {
      console.error(JSON.stringify(eventData));
    });

    let transactionInfo = null;
    try {
      transactionInfo = await transactionResult;
    } catch (error) {
      throw new Error(`Transaction failed: ${error.message}`);
    }

    const result = await transactionResult;
    console.log("Final transaction result:", result);

    //console.log("> Proof: ", proof);
    //console.log("> Nullifier: ", publicSignals);

    const calldata = await groth16.exportSolidityCallData(events, transactionResult);
    //console.log("> Raw calldata: ", calldata);

    const args = calldata.replace(/["[\]\s]/g, "").split(",");

    const formatedProof = {
      pi_a: [args[0], args[1]],
      pi_b: [
        [args[2], args[3]],
        [args[4], args[5]],
      ],
      pi_c: [args[6], args[7]],
    };
    const formatedPubSignals = args.slice(8);

    console.log("> Formated: ", formatedProof, " ", formatedPubSignals);

    console.log('Testament readed successfully:');


  }
</script>

<div class="read-testament">
  <h1>Read Testament</h1>

  <form on:submit|preventDefault={handleSubmit}>
    <div class="form-group">
      <label for="testamentId">Testament ID</label>
      <input
        type="number"
        id="testamentId"
        bind:value={formData.testamentId}
        placeholder="Enter testament ID"
        min="0"
        required
      />
    </div>
    <div class="form-group">
      <label for="passportId">Passport ID</label>
      <input
        type="text"
        id="passportId"
        bind:value={formData.passportId}
        placeholder="Enter passport ID"
        required
      />
    </div>
    <div class="form-group">
      <label for="name">Name</label>
      <input
        type="text"
        id="name"
        bind:value={formData.name}
        placeholder="Enter your Name"
        required
      />
    </div>
    <div class="form-group">
      <label for="walletAddress">Wallet Address</label>
      <input
        type="text"
        id="walletAddress"
        bind:value={formData.walletAddress}
        placeholder="Enter Wallet Address"
        required
      />
    </div>

    <button type="submit" class="primary"> View Testament </button>
  </form>
</div>

<style>
  .read-testament {
    max-width: 600px;
    margin: 0 auto;
  }

  h1 {
    color: #646cff;
    margin-bottom: 2rem;
    text-align: center;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    color: #888;
  }

  input {
    width: 100%;
    padding: 0.8rem;
    border: 1px solid #444;
    background: #2a2a2a;
    color: white;
    border-radius: 4px;
  }

  button {
    width: 100%;
    padding: 0.8rem;
    background: #646cff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
    transition: background 0.2s;
  }

  button:hover {
    background: #747bff;
  }
</style>
