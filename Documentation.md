# **Inheritance Smart Contract Project Documentation**

## **Introduction**
The Inheritance Smart Contract project is a decentralized solution that ensures secure inheritance transfers on the blockchain. By utilizing zero-knowledge proofs (ZKPs), a Merkle Tree structure, and smart contracts deployed on the Arbitrum network, this system allows users to securely create, verify, and claim inheritance without exposing sensitive information.

This document outlines the architecture, functionalities, and interaction flow between the frontend, backend, circuit verification, and smart contract.

---

## **1. System Architecture**
The project consists of four primary components:

1. **Frontend**: User interface for creating, verifying, and claiming inheritance.
2. **Backend**: Manages data, handles verification, and interacts with the blockchain.
3. **Circuits**: Zero-knowledge proof verification to ensure data integrity.
4. **Smart Contract**: Handles funds locking, verification, and distribution.

### **Technology Stack**
- **Frontend**: 
- **Backend**: 
- **Database**: JSON-based storage for off-chain data
- **Smart Contracts**: Solidity (Arbitrum Network) / future:Rust (Arbitrum Stylus)
- **Zero-Knowledge Proofs**: Circom

---

## **2. Features**
### **Creating an Inheritance**
- Users enter inheritance data (heirs, funds, and assets).
- Backend generates a Merkle Tree with the inheritance data.
- Smart contract locks the required funds.
- Data is stored in an encrypted off-chain database.

### **Reading an Inheritance**
- Users verify their identity using ZK proofs ZkVerify.
- Backend queries the database to fetch inheritance details.
- The zero-knowledge circuit verifies that the user is an eligible heir.
- Once verified, the smart contract releases the locked funds and assets.

### **Security Measures**
- **ZK Proofs**: Ensures no sensitive data is revealed during verification.
- **Merkle Tree Structure**: Provides efficient data integrity and verification.
- **Smart Contract Safeguards**: Prevents unauthorized withdrawals and ensures fair distribution.

---

## **3. Interaction Flow**
### **1. Creating Inheritance**
```mermaid
graph TD;
    User-->>Frontend: Inheritance Data
    Frontend-->>Backend: Send Data
    Backend-->>Data Base: Store in Merkle Tree
    Backend-->>Smart Contract: Lock Funds
```
1. User inputs inheritance details in the frontend.
2. The frontend sends the data to the backend.
3. The backend stores the hashed inheritance data in a Merkle Tree.
4. The backend locks funds in the smart contract corresponding to the inheritance.

### **2. Reading & Claiming Inheritance**
```mermaid
graph TD;
    User-->>Frontend: Request Verification
    Frontend-->>Backend: Send Data
    Backend-->>Data Base: Retrieve Inheritance Data
    Backend-->>Circuit: Verify ZK Proof
    Backend-->>Smart Contract: Release Funds
    Backend-->>Frontend: Confirmation Message
```
1. User requests inheritance verification.
2. The backend retrieves the stored data.
3. The circuit verifies the legitimacy of the claim using a ZK proof.
4. The smart contract releases the locked funds/assets to the rightful heir.
5. The frontend displays a success confirmation.

---


## **4. Frontend Workflow**

### **User Actions:**
- **Connect Wallet**: User connects their crypto wallet.
- **Enter Passport Data**: User inputs hashed passport data.
- **Choose Action**:
  - **Create Testament**: Enter heirs' data, assign lawyer, specify inheritance details.
  - **Read Testament**: Fetch stored JSON and convert it to PDF for display.
- **Lawyer Approval**:
  - Lawyer enters verification data.
  - Lawyer confirms user's passing and activates the testament.
- **Claim Inheritance**:
  - Heir verifies identity with ZK proof.
  - Backend retrieves testament details.
  - Smart contract releases assets upon successful verification.

---


## **5. Smart Contract Functions**
### **Structs**
```solidity
struct Heir {
    address heirAddress;
    uint256 percentage;
    string documentHash;
}

struct Testament {
    address owner;
    address lawyer;
    bool isActive;
    Heir[] heirs;
}
```

### **Key Functions**
```solidity
function createTestament(address _lawyer) external returns (uint256);
function addHeir(uint256 _testamentId, address _heir, uint256 _percentage, string memory _documentHash) external;
function activateTestament(uint256 _testamentId) external;
function viewInheritance(uint256 _testamentId) external view returns (uint256, string memory);
function claimInheritance(uint256 _testamentId) external;
```
- **`createTestament()`** - Creates a new testament.
- **`addHeir()`** - Adds heirs to the testament.
- **`activateTestament()`** - Lawyer activates the testament upon the owner's death.
- **`viewInheritance()`** - Allows heirs to view their share before activation.
- **`claimInheritance()`** - Releases inheritance funds upon activation.

---

## **6. Security Considerations**
- **Unauthorized Access Prevention**: Only the designated lawyer can activate the testament.
- **Immutability**: Testament data cannot be altered once finalized.
- **On-Chain Proofs**: Ensures the legitimacy of heir claims.
- **Circuit Verification**: Prevents fraudulent claims.

---

## **7. Future Enhancements**

- **Upgrade Rust Contract**: Improve the existing Rust contract and compare its efficiency against the Solidity smart contract.
- **NFT-Based Asset Representation**: Tokenizing physical assets in inheritance on **EDUchain** as NFTs containing all relevant information.
- **AI Integration**: Implement AI-based support for asset distribution and provide an advisory system.
- **Conditional Activation**: Enable additional conditions for contract activation (e.g., reaching age 18, getting married, etc.).
- **Multi-Sig Authentication**: Allowing multiple legal entities to approve testament activation.

---

## **8. Conclusion**
The Inheritance Smart Contract project ensures secure and verifiable asset transfer on the blockchain. By leveraging zero-knowledge proofs, a Merkle Tree structure, and smart contract automation, it eliminates intermediaries while maintaining full privacy and security. Future enhancements aim to further optimize the process and expand its functionalities.






























