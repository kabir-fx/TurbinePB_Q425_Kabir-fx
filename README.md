# Turbine Pre-builders Program Q4 2025

> A comprehensive collection of Solana development projects and Rust learning resources created as part of the Turbine Pre-builders Program 2025.

## 📚 Overview

This repository contains a journey through Solana blockchain development, from foundational Rust programming to advanced Anchor framework smart contract development. The projects demonstrate various Solana program patterns, security best practices, and full-stack dApp development.

## 🗂️ Repository Structure

### 🚀 Solana Programs (Anchor Framework)

#### 0. [**RNG_GAME**](./RNG_GAME/)
Practical Rust projects for hands-on learning.

- **Projects**:
  - `hello_world_rustc`: Basic Rust compilation
  - `hello_world_cargo`: Cargo project structure
  - `counter`: Loop implementations (loop, while, for)
  - `Rust-guessing-game`: Interactive guessing game (submodule)

#### 1. **hello_solana_anchor**
A foundational Anchor program demonstrating the basic structure and initialization pattern.

- **Features**: Simple program initialization, Anchor framework setup
- **Program ID**: [DyjRE2i1kWEJ3veCp4ohNjLHmy8LGYQZvMM1fvvvjsKg](https://explorer.solana.com/address/DyjRE2i1kWEJ3veCp4ohNjLHmy8LGYQZvMM1fvvvjsKg?cluster=devnet)
- **Tech Stack**: Anchor, Rust, TypeScript
- **Purpose**: Learn Anchor framework basics and program deployment

#### 2. **On-Chain-Visit-Tracker**
A full-stack decentralized website visit counter with Next.js frontend.

- **Features**:
  - On-chain visit tracking with unique wallet verification
  - Next.js frontend with wallet integration
  - Verifiable visit counts via Solana Explorer
  - PDA-based user tracking (one visit per wallet)
  - Tamper-proof blockchain verification
- **Live Demo**: [visit-tracker-blond.vercel.app](https://visit-tracker-blond.vercel.app/)
- **Tech Stack**: Anchor, Next.js, Tailwind CSS, Solana Web3.js
- **Key Learnings**: Full-stack dApp development, wallet integration, on-chain data verification

#### 3. **solana-mint-token-blueprint**
Comprehensive SPL token creation and minting with Metaplex metadata integration.

- **Features**:
  - Create custom SPL tokens with metadata
  - Automatic Associated Token Account (ATA) management
  - Metaplex Token Metadata v3 integration
  - Configurable mint and freeze authorities
  - 9 decimal precision support
- **Deployed Addresses** (Devnet):
  - Mint: [BikAsHyzpXGERKupnXUfe5GLX1t6cNDK1MBJ4q3ikpiS](https://explorer.solana.com/address/BikAsHyzpXGERKupnXUfe5GLX1t6cNDK1MBJ4q3ikpiS?cluster=devnet)
  - Token Account: [5ojqB6481VkTaaZA9jonJeL8xtsphk2PwxvzGNsgpgeC](https://explorer.solana.com/address/5ojqB6481VkTaaZA9jonJeL8xtsphk2PwxvzGNsgpgeC?cluster=devnet)
- **Key Learnings**: CPI calls, Metaplex integration, token standards

#### 4. **SOL-Vault**
A programmable token vault system with automatic target amount management.

- **Features**:
  - Vault initialization with configurable target amounts
  - Secure token deposits into PDA-owned vaults
  - Flexible withdrawal mechanisms
  - Automatic withdrawal when target amount is reached
- **Deployed Addresses** (Devnet):
  - Vault Address: [2edPKQcwc7ysGP7xP5EvhRcrBfGpg3WUfftvXXrm6Sxz](https://explorer.solana.com/address/2edPKQcwc7ysGP7xP5EvhRcrBfGpg3WUfftvXXrm6Sxz?cluster=devnet)
  - Vault State Address: [DWeJiiaUEdkn1YcYueu1VjgjDmoc7CxUa4qPsEr7MucX](https://explorer.solana.com/address/DWeJiiaUEdkn1YcYueu1VjgjDmoc7CxUa4qPsEr7MucX?cluster=devnet)

- **Key Learnings**: SOL transfers, PDA signing, state management

#### 5. **Escrow-Blueprint**
A secure, trustless token-to-token escrow system for atomic token exchanges.

- **Features**:
  - Create escrow offers with token deposits
  - Accept offers with matching token exchanges
  - Refund mechanism for unaccepted offers
  - PDA-based security architecture
  - Token Extensions Program support
- **Program ID**: `AJpwJAWYrFnWJuaA4aNCXgqkq5z7MsSzvXM3Xv5DamWV`
- **Tech Stack**: Anchor, SPL Token, Token Extensions
- **Key Learnings**: PDA management, token transfers, account validation

#### 6. **Core-NFT-Blueprint**
A blueprint to create custom NFT tokens and collection using Metaplex Core.

- **Features**:
  - Initialize NFT collections with metadata, update authorities, and organizational structure
  - Leverages the official Metaplex Core program for robust NFT functionality
  - Uploads images and metadata to Arweave via Irys for decentralized storage
  - Vitest-based tests with real Solana devnet interactions
- **Deployed Addresses** (Devnet):
    - NFT Mint: [FRA7M1NYjWJs8LRkt8qcrF9Gn659zmJSbnfnWeuEmoS7](https://core.metaplex.com/explorer/FRA7M1NYjWJs8LRkt8qcrF9Gn659zmJSbnfnWeuEmoS7?env=devnet)
    - Collection Mint: [CZDwqC6ex6njsAcdSKww5f9GEkKCdTfUPk4Dx1QLd2xf](https://explorer.solana.com/address/CZDwqC6ex6njsAcdSKww5f9GEkKCdTfUPk4Dx1QLd2xf?cluster=devnet)
- **Tech Stack**: Anchor, Metaplex Core, Codama
- **Key Learnings**: NFTs implementation, account management

### 📖 Learning Resources

#### 7. **Rustlings**
Comprehensive Rust programming exercises covering all fundamental concepts.

- **Content**: 100+ exercises across 23+ topics
- **Topics Covered**:
  - Variables, Functions, Control Flow
  - Structs, Enums, Pattern Matching
  - Ownership, Borrowing, Lifetimes
  - Error Handling, Generics, Traits
  - Smart Pointers, Threads, Macros
- **Structure**: Exercises with solutions included
- **Reference**: Maps to "The Rust Programming Language" book chapters

## 📁 Project Organization

```
TurbinePB_Q425_Kabir-fx/
├── Core-NFT-Blueprint/            # Core NFT Implementation
├── Escrow-Blueprint/              # Token escrow system
├── On-Chain-Visit-Tracker/        # Full-stack visit counter dApp
├── RNG_GAME/                      # Rust learning projects
├── Rustlings/                     # Rust exercises & solutions
├── SOL-Vault/                     # SOL vault with targets
├── hello_solana_anchor/           # Basic Anchor program
├── solana-mint-token-blueprint/   # Token minting with metadata
└── README.md                      # This file
```

## 🤝 Contributing

This repository is part of an educational program. For improvements:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## 📚 Additional Resources

### Official Documentation
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://www.anchor-lang.com/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [SPL Token Program](https://spl.solana.com/token)

## 🙏 Acknowledgments

- Turbine team for organizing the Pre-builders Program
- Solana Foundation for excellent documentation and tooling
- Anchor team for the fantastic development framework
- Rust community for comprehensive learning resources

---
