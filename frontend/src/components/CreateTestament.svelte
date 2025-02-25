<script lang="ts">
  import { mulPointEscalar, Base8, Point } from "@zk-kit/baby-jubjub";
  import crypto from "crypto";

  //Hardcoded for hackathon Demo
  const generateKeys = (): { privateKey: bigint; publicKey: Point<bigint> } => {
    const privateKey =
      BigInt("0x" + crypto.randomBytes(32).toString("hex")) %
      BigInt(
        "2736030358979909402780800718157159386076813972158567259200215660948447373041",
      ); // Generate random private key
    const publicKey = mulPointEscalar(Base8, privateKey); // Compute public key by scalar multiplication
    return { privateKey, publicKey };
  };

  let formData = {
    passportData: '',
    lawyerAddress: '',
    heirs: [{
      passportId: '',
      name: '',
      walletAddress: '',
      percentage: '',
      items: ''
    }]
  }

  const arbContractAddress = "0x9B0b6A4018CEc2d5191B665b0B4D4fbeB61aCeC1";

  const arbContractABI = [
  "function createTestament(address _lawyer) external returns (uint256)",
  "function addHeir(uint256 _testamentId, address _heir, uint256 _percentage, string memory _documentHash) external",
  "function getHeirs(uint256 _testamentId) external view returns (Heir[] memory)",
  "function activateTestament(uint256 _testamentId) external",
  "function viewInheritance(uint256 _testamentId) external view returns (uint256, string memory)",
  "function claimInheritance(uint256 _testamentId) external",
];


  let testamentID: bigint | null = null;
  const FIELD_SIZE = BigInt('21888242871839275222246405745257275088548364400416034343698204186575808495617');


  function addHeir() {
    formData.heirs = [...formData.heirs, {
      passportId: '',
      name: '',
      walletAddress: '',
      percentage: '',
      items: ''
    }]
  }

  async function handleSubmit(){
    try {
      const testamentData = {
        createdAt: new Date().toISOString(),
        ...formData
      }

      // Create a hash of the testament data
      const dataString = JSON.stringify(testamentData)
      const encoder = new TextEncoder()
      const IdTestament = encoder.encode(dataString)
      console.log(IdTestament)
      const hashBuffer = await crypto.subtle.digest('SHA-256', IdTestament)
      const hashArray = Array.from(new Uint8Array(hashBuffer))
      const testamentId = hashArray.map(b => b.toString(10).padStart(2, '0')).join('')
      const testamentIdBigInt = BigInt(testamentId) % FIELD_SIZE;
      
      // Add the hash as ID
      const finalTestamentData = {
        id: testamentIdBigInt.toString(),
        ...testamentData
      }

      // Generate private and public keys for the user
      const { privateKey, publicKey } = generateKeys();
      console.log("Private key is: ", privateKey);
      const publicKeyX = publicKey[0];
      const publicKeyY = publicKey[1];

      //console.log("Generated private key:", privateKey.toString(16));
      //console.log("Generated public key:", publicKeyX, publicKeyY);

      // Generate necessary data
      const data = {
        name: formData.heirs[0].name,
        pid: formData.heirs[0].passportId,
        publicKeyX: publicKeyX.toString(),
        publicKeyY: publicKeyY.toString(),
      };

      // Send the data to your backend API
      const response = await fetch('http://localhost:3000/api/testaments', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
      })

      if (!response.ok) {
        throw new Error('Failed to save testament')
      }

      testamentID = testamentIdBigInt;
      console.log(testamentID)
      console.log(testamentData)
      console.log('Testament created on blockchain and saved to backend successfully')
    } catch (error) {
      console.error('Error creating testament:', error)
      alert('Error creating testament. Check console for details.')
    }
  }
</script>

<div class="create-testament">
  <h1>Create Testament</h1>
  
  <form on:submit|preventDefault={handleSubmit}>
    <div class="form-group">
      <label for="passportData">Passport Data (will be hashed)</label>
      <input 
        type="text" 
        id="passportData" 
        bind:value={formData.passportData} 
        required
      />
    </div>

    <div class="form-group">
      <label for="lawyerAddress">Lawyer's Wallet Address</label>
      <input 
        type="text" 
        id="lawyerAddress" 
        bind:value={formData.lawyerAddress} 
        required
      />
    </div>

    <h2>Heirs</h2>
    {#each formData.heirs as heir, i}
      <div class="heir-section">
        <h3>Heir {i + 1}</h3>
        <div class="form-group">
          <label for="name{i}">Passport ID</label>
          <input 
            type="text" 
            id="passportId{i}" 
            bind:value={heir.passportId} 
            required
          />
        </div>
        <div class="form-group">
          <label for="name{i}">Name</label>
          <input 
            type="text" 
            id="name{i}" 
            bind:value={heir.name} 
            required
          />
        </div>
        <div class="form-group">
          <label for="address{i}">Wallet Address</label>
          <input 
            type="text" 
            id="address{i}" 
            bind:value={heir.walletAddress} 
            required
          />
        </div>
        <div class="form-group">
          <label for="percentage{i}">Inheritance Percentage</label>
          <input 
            type="number" 
            id="percentage{i}" 
            bind:value={heir.percentage} 
            min="0" 
            max="100" 
            required
          />
        </div>
        <div class="form-group">
          <label for="items{i}">Inherited Items (comma-separated)</label>
          <textarea 
            id="items{i}" 
            bind:value={heir.items} 
            rows="3"
          ></textarea>
        </div>
      </div>
    {/each}

    <button type="button" class="secondary" on:click={addHeir}>
      Add Another Heir
    </button>

    <button type="submit" class="primary">
      Create Testament
    </button>
  </form>
  {#if testamentID}
  <div class="testament-result">
    <h2>Testament Created Successfully</h2>
    <p><strong>ID:</strong> {testamentID}</p>
  </div>
{/if}
</div>

<style>
  .create-testament {
    max-width: 800px;
    margin: 0 auto;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    color: #888;
  }

  input, textarea {
    width: 100%;
    padding: 0.8rem;
    border: 1px solid #444;
    background: #2a2a2a;
    color: white;
    border-radius: 4px;
  }

  .heir-section {
    background: #2a2a2a;
    padding: 1.5rem;
    border-radius: 8px;
    margin-bottom: 2rem;
  }

  h1, h2, h3 {
    color: #646cff;
  }

  button {
    padding: 0.8rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
    margin-right: 1rem;
  }

  .primary {
    background: #646cff;
    color: white;
  }

  .secondary {
    background: #2a2a2a;
    color: #646cff;
    border: 1px solid #646cff;
  }

  button:hover {
    opacity: 0.9;
  }


</style> 