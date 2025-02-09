import { createAppKit } from '@reown/appkit'
import { EthersAdapter } from '@reown/appkit-adapter-ethers'
import { arbitrum, arbitrumSepolia, mainnet } from '@reown/appkit/networks'

let modal: ReturnType<typeof createAppKit> | undefined = undefined


  const projectId = '1017eb0e6d07b9e56dccfcf17bb47cef'
  if (!projectId) {
    throw new Error('VITE_WALLET_CONNECT_PROJECT_ID is not set')
  }

  modal = createAppKit({
    projectId,
    adapters: [new EthersAdapter()],
    networks: [mainnet, arbitrum, arbitrumSepolia],
    defaultNetwork: arbitrumSepolia,
    metadata: {
      name: 'ZkInheritance',
      description: 'ZkInheritance Web3 App',
      url: window.location.origin,
      icons: ['https://avatars.githubusercontent.com/u/37784886']
    }
  })


export { modal } 