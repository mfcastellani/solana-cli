//! Transaction testing and debugging

use borsh::BorshSerialize;
use sol_template_shared::{unpack_from_slice, ACCOUNT_STATE_SPACE};
use solana_cli_template_program_bpf::processor::process;
use solana_program::hash::Hash;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    transport::TransportError,
};

/// Sets up the Program test and initializes 'n' program_accounts
async fn setup(program_id: &Pubkey, program_accounts: &[Pubkey]) -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "solana_cli_template_program_bpf", // Run the BPF version with `cargo test-bpf`
        *program_id,
        processor!(process), // Run the native version with `cargo test`
    );
    for account in program_accounts {
        program_test.add_account(
            *account,
            Account {
                lamports: 5,
                data: vec![0_u8; ACCOUNT_STATE_SPACE],
                owner: *program_id,
                ..Account::default()
            },
        );
    }
    program_test.start().await
}

/// Submit transaction with relevant instruction data
#[allow(clippy::ptr_arg)]
async fn submit_txn(
    program_id: &Pubkey,
    instruction_data: &Vec<Vec<u8>>,
    accounts: &[AccountMeta],
    payer: &dyn Signer,
    recent_blockhash: Hash,
    banks_client: &mut BanksClient,
) -> Result<(), TransportError> {
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_borsh(
            *program_id,
            instruction_data,
            accounts.to_vec(),
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer], recent_blockhash);
    banks_client.process_transaction(transaction).await
}

#[tokio::test]
/// Initialization test
async fn test_initialize_pass() {
    let program_id = Pubkey::new_unique();
    let account_pubkey = Pubkey::new_unique();

    // Setup runtime testing and accounts
    let (mut banks_client, payer, recent_blockhash) = setup(&program_id, &[account_pubkey]).await;

    // Verify account is not yet initialized
    let (is_initialized, _btree_map) = match banks_client.get_account(account_pubkey).await.unwrap()
    {
        Some(account) => unpack_from_slice(&account.data).unwrap(),
        None => panic!(),
    };
    assert!(!is_initialized);

    // Initialize the account
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let initialize = vec![0u8];
    instruction_data.push(initialize);
    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());
    let (is_initialized, _btree_map) = match banks_client.get_account(account_pubkey).await.unwrap()
    {
        Some(account) => unpack_from_slice(&account.data).unwrap(),
        None => panic!(),
    };
    assert!(is_initialized);
}

#[tokio::test]
/// Mint test
async fn test_mint_pass() {
    let program_id = Pubkey::new_unique();
    let account_pubkey = Pubkey::new_unique();

    // Setup runtime testing and accounts
    let (mut banks_client, payer, recent_blockhash) = setup(&program_id, &[account_pubkey]).await;

    // Initialize the account
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let initialize = vec![0u8];
    instruction_data.push(initialize);
    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());

    // Do mint
    let mint_key = String::from("test_key_1");
    let mint_value = String::from("value for test_key_1");

    let mut instruction_data = Vec::<Vec<u8>>::new();
    let mint = vec![1u8];
    instruction_data.push(mint);
    instruction_data.push(String::try_to_vec(&mint_key).unwrap());
    instruction_data.push(String::try_to_vec(&mint_value).unwrap());

    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(account_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());
    // Check the data
    let (is_initialized, btree_map) = match banks_client.get_account(account_pubkey).await.unwrap()
    {
        Some(account) => unpack_from_slice(&account.data).unwrap(),
        None => panic!(),
    };
    assert!(is_initialized);
    assert!(btree_map.contains_key(&mint_key));
    assert_eq!(btree_map.get(&mint_key).unwrap(), &mint_value);
}

#[tokio::test]
/// Transfer test
async fn test_mint_transfer_pass() {
    let program_id = Pubkey::new_unique();
    let start_pubkey = Pubkey::new_unique();
    let target_pubkey = Pubkey::new_unique();

    // Setup runtime testing and accounts
    let (mut banks_client, payer, recent_blockhash) =
        setup(&program_id, &[start_pubkey, target_pubkey]).await;

    // Initialize the account(s)
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let initialize = vec![0u8];
    instruction_data.push(initialize);
    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(start_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());
    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(target_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());

    // Do mint
    let mint_key = String::from("test_key_1");
    let mint_value = String::from("value for test_key_1");
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let mint = vec![1u8];
    instruction_data.push(mint);
    instruction_data.push(String::try_to_vec(&mint_key).unwrap());
    instruction_data.push(String::try_to_vec(&mint_value).unwrap());

    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(start_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());

    // Do transfer
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let mint = vec![2u8];
    instruction_data.push(mint);
    instruction_data.push(String::try_to_vec(&mint_key).unwrap());
    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[
            AccountMeta::new(start_pubkey, false),
            AccountMeta::new(target_pubkey, false),
        ],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());

    let (is_initialized, btree_map) = match banks_client.get_account(start_pubkey).await.unwrap() {
        Some(account) => unpack_from_slice(&account.data).unwrap(),
        None => panic!(),
    };
    assert!(is_initialized);
    assert!(!btree_map.contains_key(&mint_key));

    let (is_initialized, btree_map) = match banks_client.get_account(target_pubkey).await.unwrap() {
        Some(account) => unpack_from_slice(&account.data).unwrap(),
        None => panic!(),
    };
    assert!(is_initialized);
    assert!(btree_map.contains_key(&mint_key));
    assert_eq!(btree_map.get(&mint_key).unwrap(), &mint_value);
}

#[tokio::test]
/// Burn test
async fn test_mint_transfer_burn_pass() {
    let program_id = Pubkey::new_unique();
    let start_pubkey = Pubkey::new_unique();
    let target_pubkey = Pubkey::new_unique();

    // Setup runtime testing and accounts
    let (mut banks_client, payer, recent_blockhash) =
        setup(&program_id, &[start_pubkey, target_pubkey]).await;

    // Initialize the account(s)
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let initialize = vec![0u8];
    instruction_data.push(initialize);
    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(start_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());
    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(target_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());

    // Do mint
    let mint_key = String::from("test_key_1");
    let mint_value = String::from("value for test_key_1");
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let mint = vec![1u8];
    instruction_data.push(mint);
    instruction_data.push(String::try_to_vec(&mint_key).unwrap());
    instruction_data.push(String::try_to_vec(&mint_value).unwrap());

    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(start_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());

    // Do transfer
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let transfer = vec![2u8];
    instruction_data.push(transfer);
    instruction_data.push(String::try_to_vec(&mint_key).unwrap());
    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[
            AccountMeta::new(start_pubkey, false),
            AccountMeta::new(target_pubkey, false),
        ],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());

    // Do the burn
    let mut instruction_data = Vec::<Vec<u8>>::new();
    let burn = vec![3u8];
    instruction_data.push(burn);
    instruction_data.push(String::try_to_vec(&mint_key).unwrap());
    let result = submit_txn(
        &program_id,
        &instruction_data,
        &[AccountMeta::new(target_pubkey, false)],
        &payer,
        recent_blockhash,
        &mut banks_client,
    )
    .await;
    assert!(result.is_ok());
}
