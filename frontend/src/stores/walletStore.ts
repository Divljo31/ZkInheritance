import { writable, derived } from 'svelte/store'
import { modal } from '../lib/appkit'

interface WalletState {
  address: string
  isConnected: boolean
  chainId: string
  chainName: string
}

function createWalletStore() {
  const { subscribe, set, update } = writable<WalletState>({
    address: '',
    isConnected: false,
    chainId: '',
    chainName: ''
  })

  return {
    subscribe,
    initialize: async () => {
      if (!modal) return
      try {
        const address = await modal.getAddress()
        const chainId = await modal.getChainId()
        
        if (address) {
          set({
            isConnected: true,
            address,
            chainId: chainId?.toString() || '',
            chainName: getChainName(chainId?.toString())
          })
        }
      } catch (error) {
        console.error('Wallet initialization error:', error)
        set({
          isConnected: false,
          address: '',
          chainId: '',
          chainName: ''
        })
      }
    },
    disconnect: async () => {
      if (!modal) return
      try {
        await modal.disconnect()
        set({
          isConnected: false,
          address: '',
          chainId: '',
          chainName: ''
        })
      } catch (error) {
        console.error('Disconnect error:', error)
      }
    },
    connect: async () => {
      if (!modal) return
      try {
        await modal.open()
        await walletStore.initialize()
      } catch (error) {
        console.error('Connection error:', error)
      }
    }
  }
}

function getChainName(chainId: string | undefined): string {
  if (!chainId) return ''
  const chains: Record<string, string> = {
    '1': 'Ethereum',
    '42161': 'Arbitrum',
    '421614': 'Arbitrum Sepolia'
  }
  return chains[chainId] || 'Unknown Network'
}

export const walletStore = createWalletStore()

export const formattedAddress = derived(
  walletStore,
  $state => $state.address ? 
    `${$state.address.slice(0, 6)}...${$state.address.slice(-4)}` : 
    ''
) 