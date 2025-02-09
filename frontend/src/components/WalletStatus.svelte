<script lang="ts">
  import { walletStore, formattedAddress } from '../stores/walletStore'
</script>

<div class="wallet-status">
  {#if $walletStore.isConnected}
    <div class="status connected">
      <span class="dot"></span>
      <div class="info">
        <p class="address">{$formattedAddress}</p>
        <p class="network">{$walletStore.chainName}</p>
      </div>
    </div>
  {:else}
    <div class="status disconnected">
      <span class="dot"></span>
      <p>Not Connected</p>
    </div>
  {/if}
</div>

<style>
  .wallet-status {
    background: #2a2a2a;
    padding: 0.8rem 1.2rem;
    border-radius: 12px;
    margin-bottom: 1rem;
  }

  .status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
  }

  .connected .dot {
    background: #4CAF50;
  }

  .disconnected .dot {
    background: #f44336;
  }

  .info {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }

  .address {
    font-family: monospace;
    margin: 0;
  }

  .network {
    font-size: 0.8em;
    color: #888;
    margin: 0;
  }
</style> 