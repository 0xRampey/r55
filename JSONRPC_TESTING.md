# R55 JSON-RPC Integration Testing

This guide explains how to test R55 ERC20 contract deployment and interaction via JSON-RPC against a local Ethereum node.

## Prerequisites

1. **Local Ethereum Node**: You need a local Ethereum node running on `localhost:8545` with R55 support
2. **Funded Test Account**: The tests use a well-known private key with pre-funded account

### Recommended Setup Options

#### Option 1: Anvil (Foundry) with R55 Support
If you have an R55-compatible version of Anvil:
```bash
anvil --accounts 10 --balance 1000
```

#### Option 2: Reth with R55 Support  
If using Reth with R55 integration:
```bash
reth node --dev --http --http.api eth,net,web3,debug,trace
```

#### Option 3: Custom R55 Node
Start your R55-enabled Ethereum node on port 8545.

## Test Structure

The JSON-RPC tests are located in `r55/tests/erc20_jsonrpc.rs` and include:

### Test Cases

1. **`test_jsonrpc_erc20_deployment()`**
   - Compiles R55 ERC20 contract to RISC-V bytecode
   - Deploys via JSON-RPC CREATE transaction
   - Verifies deployment by calling `owner()` function

2. **`test_jsonrpc_erc20_mint_and_transfer()`**
   - Deploys contract
   - Mints tokens to Alice (1000 tokens)
   - Transfers tokens from Alice to Bob (500 tokens)
   - Verifies balances via JSON-RPC calls

3. **`test_jsonrpc_erc20_approve_and_transfer_from()`**
   - Tests ERC20 approval mechanism
   - Alice approves Bob to spend tokens
   - Verifies allowance via JSON-RPC calls

4. **`test_jsonrpc_erc20_total_supply()`**
   - Tests total supply tracking
   - Verifies supply increases after minting

### Key Features

- **Real R55 Contract**: Compiles actual R55 ERC20 contract from `examples/erc20`
- **Full Integration**: Tests complete deployment and interaction flow
- **JSON-RPC Communication**: Uses alloy client for Ethereum JSON-RPC calls
- **RISC-V Bytecode**: Deploys R55 contracts with `0xFF` prefix

## Running the Tests

### 1. Start Local Node
First, start your local Ethereum node with R55 support on `localhost:8545`.

### 2. Run Individual Tests
```bash
# Run all JSON-RPC tests
cargo test --test erc20_jsonrpc

# Run specific test
cargo test --test erc20_jsonrpc test_jsonrpc_erc20_deployment

# Run with output
cargo test --test erc20_jsonrpc -- --nocapture
```

### 3. View Detailed Logs
```bash
RUST_LOG=debug cargo test --test erc20_jsonrpc -- --nocapture
```

## Expected Output

Successful test run should show:
```
✅ Deployed R55 ERC20 contract at: 0x1234...
✅ Contract deployment verified successfully!
✅ Minted 1000 tokens to Alice
✅ Transferred 500 tokens from Alice to Bob
✅ All balance checks passed!
   Alice balance: 500
   Bob balance: 500
```

## Test Accounts

The tests use these predefined accounts:

- **Alice (Deployer)**: 
  - Private Key: `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80`
  - Address: `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266`

- **Bob (Recipient)**:
  - Address: `0x70997970C51812dc3A010C7d01b50e0d17dc79C8`

These are standard Hardhat/Anvil test accounts that should be pre-funded in development environments.

## Troubleshooting

### Connection Issues
```
Error: Transport error: reqwest::Error
```
**Solution**: Ensure your local node is running on `localhost:8545`

### Insufficient Funds
```
Error: Transaction failed: insufficient funds
```
**Solution**: Ensure test accounts have sufficient ETH balance

### R55 Not Supported
```
Error: Contract deployment failed
```
**Solution**: Ensure your local node supports R55 (RISC-V) contracts

### Compilation Issues
```
Error: Failed to compile contract
```
**Solution**: Check that R55 compilation dependencies are installed:
```bash
rustup install nightly-2025-01-07
rustup target add riscv64imac-unknown-none-elf
```

## Implementation Details

### Contract Compilation
```rust
let bytecode = compile_with_prefix(compile_deploy, ERC20_PATH)?;
```
- Compiles Rust contract to RISC-V ELF
- Adds `0xFF` prefix for R55 identification
- Includes constructor arguments

### Deployment Transaction
```rust
let deploy_tx = TransactionRequest::default()
    .with_data(bytecode_with_constructor_args)
    .with_from(deployer_address);
```

### Function Calls
```rust
let calldata = [selector, args].concat();
let call = TransactionRequest::default()
    .with_to(contract_address)
    .with_data(calldata);
```

This testing setup provides comprehensive integration testing for R55 contracts in a real Ethereum environment, validating the complete deployment and execution pipeline from Rust source code to live contract interaction.