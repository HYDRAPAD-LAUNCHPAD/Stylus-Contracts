# Arbitrum Stylus Token Factory and ERC-20 Contract

This project implements a Token Factory on Arbitrum Stylus, allowing users to create their own ERC-20 tokens. The project consists of two main contracts:

1. **Token Contract**: A customizable ERC-20 token with features like minting, burning, and transferring tokens.
2. **Token Factory Contract**: A contract that allows users to deploy their own tokens through a simple interface.

## Features

### Token Contract
- **Token Metadata**: Name, Symbol, Decimals
- **Max Supply**: Fixed maximum token supply (prevents over-minting).
- **Minting**: The owner can mint new tokens up to the maximum supply.
- **Burning**: The owner can burn tokens, reducing total supply (optional via a flag).
- **Transfers**: Standard ERC-20 token transfers.
  
### Token Factory Contract
- **Create Tokens**: Allows users to create their own ERC-20 tokens with customizable features (name, symbol, max supply, etc.).
- **Owner Control**: Only the owner of the factory can create tokens.
- **Track Created Tokens**: The factory tracks tokens created by each address.

## Prerequisites

- Rust: You need to have Rust installed on your machine. If you haven't installed it yet, you can follow the installation guide [here](https://www.rust-lang.org/tools/install).
- Cargo: The Rust package manager, which comes bundled with Rust.
- `cargo-contract`: A tool for building, testing, and deploying smart contracts written in Rust. You can install it by running the following command:
  
  ```bash
  cargo install cargo-contract
  ```

- **Arbitrum Stylus Environment**: This code is intended to run on the Arbitrum Stylus network. You'll need to configure your environment to interact with Stylus, which supports EVM-compatible smart contracts.

## Installation

1. Clone the repository to your local machine:

   ```bash
   git clone https://github.com/your-repository-url.git
   cd your-repository-folder
   ```

2. Install the required dependencies:

   ```bash
   cargo build
   ```

3. Build the contract:

   ```bash
   cargo contract build
   ```

4. Deploy the contract to a local or test network using the following command:

   ```bash
   cargo contract deploy --url <network-url> --signer <signer> --contract <contract-name>
   ```

   Replace `<network-url>`, `<signer>`, and `<contract-name>` with the appropriate values for your setup.

## Contract Details

### Token Contract (`token.rs`)

This contract implements a basic ERC-20 token that can be minted and burned (if enabled). It supports the following functions:

- **`new()`**: Constructor that initializes the token with its name, symbol, decimals, maximum supply, and burnable flag.
- **`total_supply()`**: Returns the total supply of tokens in circulation.
- **`balance_of(account)`**: Returns the balance of tokens for a given account.
- **`transfer(to, value)`**: Transfers tokens from the caller to another account.
- **`approve(spender, value)`**: Approves a spender to transfer a specific amount of tokens on behalf of the caller.
- **`mint(to, value)`**: Allows the owner to mint new tokens (up to the maximum supply).
- **`burn(value)`**: Allows the owner to burn a specified number of tokens, reducing the total supply.

### Token Factory Contract (`factory.rs`)

This contract allows the creation of new tokens and tracks the tokens created by each user. It includes the following features:

- **`new()`**: Initializes the factory contract.
- **`create_token()`**: Allows the owner of the factory to create a new token with specified parameters (name, symbol, max supply, etc.).
- **`get_created_token(creator)`**: Retrieves the address of the token created by a specific user.

## Security Features

- **Owner-only Access**: Only the owner of the factory can create tokens and mint new tokens.
- **Max Supply Enforcement**: Tokens cannot be minted beyond the maximum supply.
- **Burn Restrictions**: Burning tokens can only be performed by the owner if the token is marked as burnable.
- **Safe Transfers**: Transfers are checked for sufficient balance to prevent over-transfers.
- **Storage**: Token balances and contract states are securely stored using the `ink_storage` library.

## Usage

### Interacting with the Factory

1. **Deploy the Factory Contract**:
   - First, deploy the `TokenFactory` contract. This contract should be deployed by a trusted account (the owner).

2. **Create Tokens**:
   - Users can call the `create_token()` method on the `TokenFactory` contract to deploy a new token. They will need to provide the following parameters:
     - `name`: The name of the token (e.g., "MyToken").
     - `symbol`: The symbol of the token (e.g., "MTK").
     - `decimals`: The number of decimals for the token (usually 18).
     - `max_supply`: The maximum number of tokens that can ever be minted.
     - `burnable`: Whether or not the token can be burned.

   Example:
   ```rust
   factory.create_token("MyToken", "MTK", 18, 1_000_000_000, true);
   ```

3. **Interact with Tokens**:
   - Once the token is created, users can interact with it using its ERC-20 functions, such as `transfer()`, `mint()`, `burn()`, and `balance_of()`.
   - The owner of the token contract has special privileges, such as minting and burning tokens.

### Example Usage

Once deployed, users can interact with the token as follows:

- **Transfer tokens**:

  ```rust
  token.transfer(to_account_id, amount);
  ```

- **Mint new tokens** (only by the owner):

  ```rust
  token.mint(to_account_id, amount);
  ```

- **Burn tokens** (only by the owner if enabled):

  ```rust
  token.burn(amount);
  ```

## Testing

You can test the contract using the `cargo-contract` testing framework. Create unit tests within the `tests` folder to interact with the deployed contracts.

```bash
cargo test
```

## Deployment on Arbitrum Stylus

1. **Deploying the Factory**: Once the factory is deployed, users can create their own tokens via the factory’s `create_token()` method.
2. **Interacting with Tokens**: After deployment, token owners can interact with their token’s contract using the ERC-20 methods (`transfer`, `mint`, `burn`).
   
The contracts are ready to be deployed to the Arbitrum Stylus network, which supports Ethereum-compatible smart contracts.

