<script lang="ts">
  let testamentId = ''

  async function handleSubmit() {
    if (!testamentId) return
    console.log('Fetching testament with ID:', testamentId)
    // TODO: Implement contract interaction to fetch testament details
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
        bind:value={testamentId} 
        placeholder="Enter testament ID"
        min="0"
        required
      />
    </div>

    <button type="submit" class="primary">
      View Testament
    </button>
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