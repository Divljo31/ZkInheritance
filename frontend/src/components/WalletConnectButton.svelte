<script lang="ts">
  import { modal } from '../lib/appkit'
  import { walletStore } from '../stores/walletStore'
  import WalletStatus from './WalletStatus.svelte'
  import { onMount } from 'svelte'

  onMount(() => {
    if (!modal) return
    const unsubscribe = modal.subscribeProviders(() => {
      walletStore.initialize()
    })
    
    walletStore.initialize()
    return unsubscribe
  })
</script>

<div class="wallet-connect">
  {#if $walletStore.isConnected}
    <button on:click={() => walletStore.disconnect()}>Disconnect Wallet</button>
  {:else}
    <button on:click={() => walletStore.connect()}>Connect Wallet</button>
  {/if}
</div>

<style>
  .wallet-connect {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  button {
    background: #646cff;
    color: white;
    border: none;
    padding: 0.8rem 1.2rem;
    border-radius: 8px;
    cursor: pointer;
    font-size: 1rem;
    transition: background 0.2s;
  }

  button:hover {
    background: #747bff;
  }
</style>