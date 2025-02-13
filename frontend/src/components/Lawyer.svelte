<script lang="ts">
  import { ethers } from 'ethers';
  import { modal } from '../lib/appkit'
  import { onMount } from 'svelte';


  const arbContractAddress = "0x9B0b6A4018CEc2d5191B665b0B4D4fbeB61aCeC1";

  const arbContractABI = [
  "function createTestament(address _lawyer) external returns (uint256)",
  "function addHeir(uint256 _testamentId, address _heir, uint256 _percentage, string memory _documentHash) external",
  "function getHeirs(uint256 _testamentId) external view returns (Heir[] memory)",
  "function activateTestament(uint256 _testamentId) external",
  "function viewInheritance(uint256 _testamentId) external view returns (uint256, string memory)",
  "function claimInheritance(uint256 _testamentId) external",
  "function testaments(uint256) public view returns (address owner, address lawyer, bool isActive)",
];
  // Mock data - replace with actual contract data later
  let testaments = [
    {
      id: 0,
      owner: '0x1234...5678',
      heirs: 2,
      lawyer: '0x1234...5678',
      isActive: false,
      createdAt: '2024-03-15'
    },
    {
      id: 1,
      owner: '0x8765...4321',
      heirs: 3,
      lawyer: '0x1234...5678',
      isActive: true,
      createdAt: '2024-03-16'
    }
  ];

  let selectedTestament: any = null
  let isDetailsModalOpen = false

  async function approveTestament(id: number) {
    console.log('Approving testament:', id);
   
    const provider = new ethers.BrowserProvider(await modal.getProvider('eip155'))
      
      if (!provider) {
        throw new Error('Failed to get provider')
      }
    
    const signer = await provider.getSigner()
      
      // Create contract instance
      const arbContract = new ethers.Contract(
        arbContractAddress, 
        arbContractABI, 
        signer
      )

      const tx = await arbContract.activateTestament(id);
      await tx.wait();
      console.log(tx.receipt)

    // TODO: Implement contract interaction for approval
  }

  async function viewTestamentDetails(testament: any) {
    try {
      // Get the provider using the Reown AppKit adapter
      const provider = new ethers.BrowserProvider(await modal.getProvider('eip155'))
      
      if (!provider) {
        throw new Error('Failed to get provider')
      }

      // Get signer from provider
      const signer = await provider.getSigner()
      
      // Create contract instance
      const arbContract = new ethers.Contract(
        arbContractAddress, 
        arbContractABI, 
        signer
      )

      const testamentData = await arbContract.testaments(3)
      const heirs = await arbContract.getHeirs(3)
      
      selectedTestament = {
        owner: testamentData.owner,
        lawyer: testamentData.lawyer,
        isActive: testamentData.isActive,
        heirs: heirs.map((heir: any) => ({
          address: heir.heirAddress,
          percentage: heir.percentage.toString(),
          documentHash: heir.documentHash
        }))
      }
    
      testaments.push(selectedTestament)
      isDetailsModalOpen = true
    } catch (error) {
      console.error('Error fetching testament details:', error)
      // TODO: Show error message to user
    }
  }

  async function fetchTestaments() {
    try {
      const provider = new ethers.BrowserProvider(await modal.getProvider('eip155'))
      
      if (!provider) {
        throw new Error('Failed to get provider')
      }

      const signer = await provider.getSigner()
      
      const arbContract = new ethers.Contract(
        arbContractAddress, 
        arbContractABI, 
        signer
      )

      // Get the total number of testaments (if available in your contract)
      // For now, let's try to fetch testament with ID 3
      const testamentData = await arbContract.testaments(3)
     // const heirs = await arbContract.getHeirs(3)
      
      const testament = {
        id: 3,
        owner: testamentData.owner,
        lawyer: testamentData.lawyer,
        isActive: testamentData.isActive,
        heirs: 1,
        createdAt: new Date().toISOString().split('T')[0] // You might want to get this from contract events
      }
      
     testaments = [testament]

    } catch (error) {
      console.error('Error fetching testaments:', error)
    }
  }

  onMount(() => {
    fetchTestaments();
  });
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
              <p><strong>Lawyer:</strong> {testament.lawyer}</p>
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

{#if isDetailsModalOpen && selectedTestament}
  <div class="modal">
    <div class="modal-content">
      <h2>Testament Details</h2>
      <p><strong>Owner:</strong> {selectedTestament.owner}</p>
      <p><strong>Lawyer:</strong> {selectedTestament.lawyer}</p>
      <p><strong>Status:</strong> {selectedTestament.isActive ? 'Active' : 'Pending'}</p>
      
      <h3>Heirs</h3>
      {#if selectedTestament.heirs.length > 0}
        <ul>
          {#each selectedTestament.heirs as heir}
            <li>
              <p>Address: {heir.address}</p>
              <p>Percentage: {heir.percentage}%</p>
              <p>Document Hash: {heir.documentHash}</p>
            </li>
          {/each}
        </ul>
      {:else}
        <p>No heirs assigned</p>
      {/if}
      
      <button on:click={() => isDetailsModalOpen = false}>Close</button>
    </div>
  </div>
{/if}

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

  .modal {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.8);
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .modal-content {
    background: #2a2a2a;
    padding: 2rem;
    border-radius: 8px;
    max-width: 600px;
    width: 90%;
    max-height: 90vh;
    overflow-y: auto;
  }

  .modal h2 {
    color: #646cff;
    margin-bottom: 1rem;
  }

  .modal h3 {
    color: #646cff;
    margin: 1rem 0;
  }

  .modal ul {
    list-style: none;
    padding: 0;
  }

  .modal li {
    background: #333;
    padding: 1rem;
    margin-bottom: 0.5rem;
    border-radius: 4px;
  }

  .modal p {
    margin: 0.5rem 0;
  }
</style> 