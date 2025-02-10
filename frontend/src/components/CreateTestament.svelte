<script lang="ts">
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

  let testamentID: bigint | null = null;

  function addHeir() {
    formData.heirs = [...formData.heirs, {
      passportId: '',
      name: '',
      walletAddress: '',
      percentage: '',
      items: ''
    }]
  }

  async function handleSubmit() {
    try {
      // Create the testament data object without ID first
      const testamentData = {
        createdAt: new Date().toISOString(),
        ...formData
      }

      // Create a hash of the testament data
      const dataString = JSON.stringify(testamentData)
      const encoder = new TextEncoder()
      const data = encoder.encode(dataString)
      console.log(data)
      const hashBuffer = await crypto.subtle.digest('SHA-256', data)
      const hashArray = Array.from(new Uint8Array(hashBuffer))
      const testamentId = hashArray.map(b => b.toString(10).padStart(2, '0')).join('')
      
      // Add the hash as ID
      const finalTestamentData = {
        id: testamentId,
        ...testamentData
      }

      // Send the data to your backend API
      const response = await fetch('http://localhost:3000/api/testaments', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(finalTestamentData)
      })

      if (!response.ok) {
        throw new Error('Failed to save testament')
      }

      testamentID = BigInt(testamentId);

      console.log('Testament saved successfully:', testamentId)
    } catch (error) {
      console.error('Error saving testament:', error)
      // You might want to show an error message to the user here
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