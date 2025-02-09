<script lang="ts">

  // Mock data - replace with actual contract data later
  let testaments = [
    {
      id: 0,
      owner: '0x1234...5678',
      heirs: 2,
      isActive: false,
      createdAt: '2024-03-15'
    },
    {
      id: 1,
      owner: '0x8765...4321',
      heirs: 3,
      isActive: false,
      createdAt: '2024-03-16'
    }
  ];

  async function approveTestament(id: number) {
    console.log('Approving testament:', id);
    // TODO: Implement contract interaction for approval
  }

  async function viewTestamentDetails(testament: any) {
    console.log('Viewing testament details:', JSON.stringify(testament));
    // TODO: Implement testament details view
  }
</script>

<div class="lawyer">
  <h1>Lawyer Dashboard</h1>
  
  {#if testaments.length === 0}
    <p class="no-testaments">No testaments found requiring approval</p>
  {:else}
    <div class="testaments-list">
      {#each testaments as testament}
        <div class="testament-card">
          <div class="testament-info">
            <h3>Testament #{testament.id}</h3>
            <div class="details">
              <p><strong>Owner:</strong> {testament.owner}</p>
              <p><strong>Heirs:</strong> {testament.heirs}</p>
              <p><strong>Created:</strong> {testament.createdAt}</p>
              <p class="status">
                <span class={testament.isActive ? 'active' : 'pending'}>
                  {testament.isActive ? 'Active' : 'Pending Approval'}
                </span>
              </p>
            </div>
          </div>
          <div class="actions">
            <button 
              class="view-btn"
              on:click={() => viewTestamentDetails(testament)}
            >
              View Details
            </button>
            {#if !testament.isActive}
              <button 
                class="approve-btn"
                on:click={() => approveTestament(testament.id)}
              >
                Approve
              </button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .lawyer {
    max-width: 800px;
    margin: 0 auto;
  }

  h1 {
    color: #646cff;
    margin-bottom: 2rem;
    text-align: center;
  }

  .no-testaments {
    text-align: center;
    color: #888;
    padding: 2rem;
  }

  .testaments-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .testament-card {
    background: #2a2a2a;
    border-radius: 8px;
    padding: 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }

  .testament-info {
    flex: 1;
  }

  h3 {
    color: #646cff;
    margin: 0 0 1rem 0;
  }

  .details {
    display: grid;
    gap: 0.5rem;
  }

  .details p {
    margin: 0;
    color: #888;
  }

  .details strong {
    color: #fff;
  }

  .status {
    margin-top: 0.5rem;
  }

  .status span {
    padding: 0.25rem 0.75rem;
    border-radius: 1rem;
    font-size: 0.875rem;
  }

  .pending {
    background: #472a1a;
    color: #ffa07a;
  }

  .active {
    background: #1a472a;
    color: #90ee90;
  }

  .actions {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  button {
    padding: 0.6rem 1.2rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
    transition: all 0.2s;
  }

  .view-btn {
    background: #2d2d2d;
    color: #646cff;
    border: 1px solid #646cff;
  }

  .approve-btn {
    background: #646cff;
    color: white;
  }

  button:hover {
    transform: translateY(-1px);
    opacity: 0.9;
  }

  @media (max-width: 600px) {
    .testament-card {
      flex-direction: column;
      align-items: stretch;
    }

    .actions {
      flex-direction: row;
      justify-content: stretch;
    }

    button {
      flex: 1;
    }
  }
</style> 