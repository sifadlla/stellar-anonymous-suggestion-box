# Anonymous Suggestion Box DApp

**Anonymous Suggestion Box** - A decentralized, blockchain-based platform for anonymous feedback, built on the Stellar network using the Soroban Smart Contract SDK.

## Project Description
The Anonymous Suggestion Box is a decentralized application designed to facilitate honest and open feedback. By leveraging the immutability and transparency of the Stellar blockchain, this platform allows users to submit suggestions anonymously without the risk of censorship or tampering. Every suggestion is timestamped and permanently recorded, ensuring that feedback is preserved securely and accessible for review.

## Project Vision
Our goal is to create a secure space for honest communication by removing the barriers of centralized control. We believe that by decentralizing the feedback process, organizations and communities can foster a culture of transparency, trust, and accountability.

## Key Features
- **Anonymous Submission**: Users can share thoughts without revealing their identity directly on the contract storage.
- **Timestamping**: Every suggestion is automatically tagged with a ledger timestamp for chronological tracking.
- **Efficient Retrieval**: Easily fetch the full history of suggestions in a single call.
- **Decentralized Storage**: All data is stored on-chain, ensuring high availability and resistance to unauthorized deletion (unless managed via protocol).
- **Stellar-Powered**: Built on the fast, low-cost, and scalable Stellar network.

## Contract Details
- **Network**: Stellar Testnet
- **Contract Address**: `CCD4NYHLJG3VMEFB74XY3MXZA3TMLFC7ZMMMPMOBVCQHWV3T2W6UNPA2`
- **Explorer Link**: [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CCD4NYHLJG3VMEFB74XY3MXZA3TMLFC7ZMMMPMOBVCQHWV3T2W6UNPA2)

## Testnet Interface
*(Insert a screenshot of your terminal output or your frontend interface here)*
![Anonymous Suggestion Box Screenshot](path/to/your/screenshot.png)

## Getting Started
To interact with the smart contract, ensure you have the Soroban CLI installed.

### Prerequisites
- [Stellar Soroban CLI](https://developers.stellar.org/docs/tools/sdks/cli)
- Rust environment

### Invocation Commands
1. **Submit a Suggestion**:
   ```bash
   soroban contract invoke --id CCD4NYHLJG3VMEFB74XY3MXZA3TMLFC7ZMMMPMOBVCQHWV3T2W6UNPA2 --fn submit_suggestion --arg "Your suggestion here"

ID smart contract: CCD4NYHLJG3VMEFB74XY3MXZA3TMLFC7ZMMMPMOBVCQHWV3T2W6UNPA2

