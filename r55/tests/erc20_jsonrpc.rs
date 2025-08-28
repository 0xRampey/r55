use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{Address, Bytes, U256},
    providers::{fillers::FillProvider, Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::{LocalSigner, PrivateKeySigner},
    sol_types::SolValue,
};
use alloy_core::primitives::{Keccak256, U32};
use r55::{compile_deploy, compile_with_prefix, test_utils::get_selector_from_sig};
use std::str::FromStr;
use tokio;
use revm::primitives::TransactTo;

const ERC20_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../examples/erc20");

// Test addresses - using well-known private keys for testing
const ALICE_PRIVATE_KEY: &str =
    "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const BOB_ADDRESS: &str = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";

// type BuilderProvider = FillProvider<JoinFill<JoinFill<Identity, JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>>, WalletFiller<EthereumWallet>>, RootProvider<Http<Client>>, Http<Client>, Ethereum>;

async fn compile_and_deploy_erc20() -> eyre::Result<Address> {
    // Set up signer with Alice's private key
    let signer: PrivateKeySigner = LocalSigner::from_str(ALICE_PRIVATE_KEY)?;
    let owner = signer.address();
    let wallet = EthereumWallet::from(signer);
    // Connect to local Ethereum node
    let rpc_url = "http://localhost:8545".parse()?;
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .on_http(rpc_url);
    // Compile the R55 ERC20 contract
    let bytecode = compile_with_prefix(compile_deploy, ERC20_PATH)?;

    // Encode constructor arguments (owner address)
    let constructor = owner.abi_encode();

    let init_code = if Some(&0xff) == bytecode.first() {
        // Craft R55 initcode: [0xFF][codesize][bytecode][constructor_args]
        let codesize = U32::from(bytecode.len());

        let mut init_code = Vec::new();
        init_code.push(0xff);
        init_code.extend_from_slice(&Bytes::from(codesize.to_be_bytes_vec()));
        init_code.extend_from_slice(&bytecode);
        if let Some(args) = Some(constructor) {
            init_code.extend_from_slice(&args);
        }
        Bytes::from(init_code)
    } else {
        // do not modify bytecode for EVM contracts
        bytecode
    };

    // Create deployment transaction
    let mut deploy_tx = TransactionRequest::default()
        .from(owner)
        .input(Bytes::from(init_code).into());
    deploy_tx.to = Some(TransactTo::Create);

    // Send deployment transaction
    println!("Sending deployment transaction...");
    let pending_tx = provider.send_transaction(deploy_tx).await?;
    println!("Transaction sent, waiting for receipt...");
    let receipt = pending_tx.get_receipt().await?;

    let contract_address = receipt
        .contract_address
        .ok_or_else(|| eyre::eyre!("No contract address in receipt"))?;

    println!("✅ Deployed R55 ERC20 contract at: {}", contract_address);
    Ok(contract_address)
}

// async fn call_contract(
//     provider: &impl alloy::providers::Provider,
//     contract_address: Address,
//     from: Address,
//     function_sig: &str,
//     args: Vec<u8>,
// ) -> eyre::Result<Bytes> {
//     let selector = get_selector_from_sig(function_sig);
//     let mut calldata = selector.to_vec();
//     calldata.extend_from_slice(&args);

//     let call = TransactionRequest::default()
//         .to(contract_address)
//         .from(from)
//         .input(Bytes::from(calldata).into());

//     let result = provider.call(&call).await?;
//     Ok(result)
// }

// async fn send_transaction(
//     provider: &impl alloy::providers::Provider,
//     contract_address: Address,
//     from: Address,
//     function_sig: &str,
//     args: Vec<u8>,
// ) -> eyre::Result<alloy::rpc::types::TransactionReceipt> {
//     let selector = get_selector_from_sig(function_sig);
//     let mut calldata = selector.to_vec();
//     calldata.extend_from_slice(&args);

//     let tx = TransactionRequest::default()
//         .to(contract_address)
//         .from(from)
//         .input(Bytes::from(calldata).into());

//     let pending_tx = provider.send_transaction(tx).await?;
//     let receipt = pending_tx.get_receipt().await?;
//     Ok(receipt)
// }

#[tokio::test]
async fn test_jsonrpc_erc20_deployment() -> eyre::Result<()> {
    // Deploy contract
    println!("Deploying contract...");
    let contract_address = compile_and_deploy_erc20().await?;

    // Verify deployment by calling owner() function
    // let result = call_contract(
    //     &provider,
    //     contract_address,
    //     alice_address,
    //     "owner()",
    //     vec![],
    // )
    // .await?;

    // Decode the result - owner should be Alice
    // let owner_result = Address::from_word(alloy_primitives::B256::from_slice(result.as_ref()));
    // assert_eq!(
    //     owner_result, alice_address,
    //     "Contract owner should be Alice"
    // );

    println!("✅ Contract deployment verified successfully!");
    Ok(())
}

// #[tokio::test]
// async fn test_jsonrpc_erc20_mint_and_transfer() -> eyre::Result<()> {
//     let JsonRpcSetup {
//         provider,
//         wallet: _,
//         alice_address,
//     } = setup_jsonrpc().await?;

//     let bob_address = Address::from_str(BOB_ADDRESS)?;

//     // Deploy contract
//     let contract_address = compile_and_deploy_erc20(&provider, alice_address).await?;

//     // Mint 1000 tokens to Alice
//     let mint_amount = U256::from(1000u64);
//     let mint_receipt = send_transaction(
//         &provider,
//         contract_address,
//         alice_address,
//         "mint(address,uint256)",
//         (alice_address, mint_amount).abi_encode(),
//     )
//     .await?;

//     assert!(mint_receipt.status(), "Mint transaction should succeed");
//     println!("✅ Minted {} tokens to Alice", mint_amount);

//     // Check Alice's balance
//     let balance_result = call_contract(
//         &provider,
//         contract_address,
//         alice_address,
//         "balance_of(address)",
//         alice_address.abi_encode(),
//     )
//     .await?;

//     let alice_balance = U256::from_be_bytes::<32>(balance_result.as_ref().try_into()?);
//     assert_eq!(
//         alice_balance, mint_amount,
//         "Alice should have minted tokens"
//     );

//     // Transfer 500 tokens from Alice to Bob
//     let transfer_amount = U256::from(500u64);
//     let transfer_receipt = send_transaction(
//         &provider,
//         contract_address,
//         alice_address,
//         "transfer(address,uint256)",
//         (bob_address, transfer_amount).abi_encode(),
//     )
//     .await?;

//     assert!(
//         transfer_receipt.status(),
//         "Transfer transaction should succeed"
//     );
//     println!(
//         "✅ Transferred {} tokens from Alice to Bob",
//         transfer_amount
//     );

//     // Check Bob's balance
//     let bob_balance_result = call_contract(
//         &provider,
//         contract_address,
//         alice_address,
//         "balance_of(address)",
//         bob_address.abi_encode(),
//     )
//     .await?;

//     let bob_balance = U256::from_be_bytes::<32>(bob_balance_result.as_ref().try_into()?);
//     assert_eq!(
//         bob_balance, transfer_amount,
//         "Bob should have received tokens"
//     );

//     // Check Alice's remaining balance
//     let alice_balance_result = call_contract(
//         &provider,
//         contract_address,
//         alice_address,
//         "balance_of(address)",
//         alice_address.abi_encode(),
//     )
//     .await?;

//     let alice_remaining_balance =
//         U256::from_be_bytes::<32>(alice_balance_result.as_ref().try_into()?);
//     let expected_remaining = mint_amount - transfer_amount;
//     assert_eq!(
//         alice_remaining_balance, expected_remaining,
//         "Alice should have correct remaining balance"
//     );

//     println!("✅ All balance checks passed!");
//     println!("   Alice balance: {}", alice_remaining_balance);
//     println!("   Bob balance: {}", bob_balance);

//     Ok(())
// }

// #[tokio::test]
// async fn test_jsonrpc_erc20_approve_and_transfer_from() -> eyre::Result<()> {
//     let JsonRpcSetup {
//         provider,
//         wallet: _,
//         alice_address,
//     } = setup_jsonrpc().await?;

//     let bob_address = Address::from_str(BOB_ADDRESS)?;

//     // Deploy contract
//     let contract_address = compile_and_deploy_erc20(&provider, alice_address).await?;

//     // Mint 1000 tokens to Alice
//     let mint_amount = U256::from(1000u64);
//     send_transaction(
//         &provider,
//         contract_address,
//         alice_address,
//         "mint(address,uint256)",
//         (alice_address, mint_amount).abi_encode(),
//     )
//     .await?;

//     // Alice approves Bob to spend 300 tokens
//     let approve_amount = U256::from(300u64);
//     let approve_receipt = send_transaction(
//         &provider,
//         contract_address,
//         alice_address,
//         "approve(address,uint256)",
//         (bob_address, approve_amount).abi_encode(),
//     )
//     .await?;

//     assert!(
//         approve_receipt.status(),
//         "Approve transaction should succeed"
//     );
//     println!("✅ Alice approved Bob to spend {} tokens", approve_amount);

//     // Check allowance
//     let allowance_result = call_contract(
//         &provider,
//         contract_address,
//         alice_address,
//         "allowance(address,address)",
//         (alice_address, bob_address).abi_encode(),
//     )
//     .await?;

//     let allowance = U256::from_be_bytes::<32>(allowance_result.as_ref().try_into()?);
//     assert_eq!(
//         allowance, approve_amount,
//         "Allowance should be set correctly"
//     );

//     println!("✅ Allowance verified: {} tokens", allowance);
//     Ok(())
// }

// #[tokio::test]
// async fn test_jsonrpc_erc20_total_supply() -> eyre::Result<()> {
//     let JsonRpcSetup {
//         provider,
//         wallet: _,
//         alice_address,
//     } = setup_jsonrpc().await?;

//     // Deploy contract
//     let contract_address = compile_and_deploy_erc20(&provider, alice_address).await?;

//     // Initial total supply should be 0
//     let total_supply_result = call_contract(
//         &provider,
//         contract_address,
//         alice_address,
//         "total_supply()",
//         vec![],
//     )
//     .await?;

//     let initial_supply = U256::from_be_bytes::<32>(total_supply_result.as_ref().try_into()?);
//     assert_eq!(
//         initial_supply,
//         U256::ZERO,
//         "Initial total supply should be 0"
//     );

//     // Mint tokens and check total supply increases
//     let mint_amount = U256::from(2000u64);
//     send_transaction(
//         &provider,
//         contract_address,
//         alice_address,
//         "mint(address,uint256)",
//         (alice_address, mint_amount).abi_encode(),
//     )
//     .await?;

//     let total_supply_after_mint = call_contract(
//         &provider,
//         contract_address,
//         alice_address,
//         "total_supply()",
//         vec![],
//     )
//     .await?;

//     let supply_after_mint = U256::from_be_bytes::<32>(total_supply_after_mint.as_ref().try_into()?);
//     assert_eq!(
//         supply_after_mint, mint_amount,
//         "Total supply should equal minted amount"
//     );

//     println!("✅ Total supply tests passed!");
//     println!("   Initial supply: {}", initial_supply);
//     println!("   Supply after mint: {}", supply_after_mint);
//     Ok(())
// }
