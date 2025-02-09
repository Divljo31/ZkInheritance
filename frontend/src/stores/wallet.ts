import { writable, derived } from 'svelte/store'

interface WalletState {
  address: string
  isConnected: boolean
  chainId: string
  chainName: string
}

export const walletState = writable<WalletState>({
  address: '',
  isConnected: false,
  chainId: '',
  chainName: '',
})

// Derived store for formatted address (0x1234...5678)
export const formattedAddress = derived(
  walletState,
  $state => $state.address ? 
    `${$state.address.slice(0, 6)}...${$state.address.slice(-4)}` : 
    ''
)

// Helper function to update wallet state
export function updateWalletState(newState: Partial<WalletState>) {
  console.log('Updating wallet state:', newState) // Debug log
  walletState.update(state => ({ ...state, ...newState }))
} 