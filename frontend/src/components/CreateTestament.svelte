<script lang="ts">
  export let currentView: string
  
  let formData = {
    passportData: '',
    lawyerAddress: '',
    heirs: [{
      name: '',
      walletAddress: '',
      percentage: '',
      items: ''
    }]
  }

  function addHeir() {
    formData.heirs = [...formData.heirs, {
      name: '',
      walletAddress: '',
      percentage: '',
      items: ''
    }]
  }

  function handleSubmit() {
    console.log('Testament data:', JSON.stringify(formData))
    // TODO: Implement contract interaction
    currentView = 'home'
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