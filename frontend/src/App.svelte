<script lang="ts">
  import { walletStore, formattedAddress } from './stores/walletStore'
  import { modal } from './lib/appkit'
  import { onMount } from 'svelte'
  import CreateTestament from './components/CreateTestament.svelte'
  import ReadTestament from './components/ReadTestament.svelte'
  import Lawyer from './components/Lawyer.svelte'

  let currentView = 'home'

  onMount(() => {
    if (!modal) return
    const unsubscribe = modal.subscribeProviders(() => {
      walletStore.initialize()
    })
    
    walletStore.initialize()
    return unsubscribe
  })
</script>

<div class="home">
  <div class="content">
    <h1>ZkInheritance</h1>
    <p>Create and manage your digital testament securely</p>
    
    <div class="connect-section">
      {#if $walletStore.isConnected}
        <div class="wallet-controls">
          <button class="connected">
            {$formattedAddress}
          </button>
          <button on:click={() => walletStore.disconnect()} class="disconnect">
            Disconnect
          </button>
        </div>
        
        {#if currentView === 'home'}
          <div class="menu-buttons">
            <button class="menu-btn" on:click={() => currentView = 'create-testament'}>
              Create Testament
            </button>
            <button class="menu-btn" on:click={() => currentView = 'read-testament'}>
              Read Testament
            </button>
            <button class="menu-btn" on:click={() => currentView = 'lawyer'}>
              Lawyer
            </button>
          </div>
        {:else if currentView === 'create-testament'}
          <div class="view-container">
            <button class="back-btn" on:click={() => currentView = 'home'}>← Back</button>
            <CreateTestament bind:currentView />
          </div>
        {:else if currentView === 'read-testament'}
          <div class="view-container">
            <button class="back-btn" on:click={() => currentView = 'home'}>← Back</button>
            <ReadTestament  />
          </div>
        {:else if currentView === 'lawyer'}
          <div class="view-container">
            <button class="back-btn" on:click={() => currentView = 'home'}>← Back</button>
            <Lawyer />
          </div>
        {/if}
      {:else}
        <button on:click={() => walletStore.connect()} class="connect-btn">
          Connect Wallet
        </button>
      {/if}
    </div>
  </div>
</div>

<style>
  .home {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
    padding: 2rem;
  }

  .content {
    text-align: center;
    max-width: 600px;
  }

  h1 {
    color: #646cff;
    font-size: 3.5rem;
    margin-bottom: 1rem;
  }

  p {
    color: #888;
    margin-bottom: 3rem;
    font-size: 1.2rem;
  }

  .connect-section {
    background: #2a2a2a;
    padding: 2rem;
    border-radius: 12px;
  }

  .wallet-controls {
    display: flex;
    gap: 0.75rem;
    justify-content: center;
    margin-bottom: 1.5rem;
  }

  button {
    padding: 0.75rem 1.5rem;
    font-size: 0.95rem;
    font-weight: 500;
    cursor: pointer;
    border-radius: 12px;
    border: none;
    background: #646cff;
    color: white;
    transition: all 0.2s ease;
  }

  button:hover {
    background: #747bff;
    transform: translateY(-1px);
  }

  .connect-btn {
    font-weight: 600;
    padding: 1rem 2rem;
  }

  button.connected {
    background: #1a472a;
  }

  button.disconnect {
    background: #472a1a;
  }

  button.disconnect:hover {
    background: #5a341f;
  }

  .menu-buttons {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-top: 1.5rem;
  }

  .menu-btn {
    width: 100%;
    background: #2d2d2d;
    border: 1px solid #444;
  }

  .menu-btn:hover {
    background: #383838;
  }

  .view-container {
    width: 100%;
    margin-top: 1rem;
  }

  .back-btn {
    background: transparent;
    border: none;
    color: #646cff;
    padding: 0.5rem 0;
    margin-bottom: 1rem;
    font-size: 1rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .back-btn:hover {
    color: #747bff;
    background: transparent;
    transform: none;
  }
</style>



