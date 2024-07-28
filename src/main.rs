use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::{keypair, EncodableKeypair};
use solana_sdk::instruction::{Instruction, AccountMeta};
use solana_sdk::transaction::Transaction;
use solana_sdk::system_program;
use serde::{Serialize, Deserialize};
use std::env;

const PROGRAM_ID: &str = "imgZzuUv47Wwy6aV39mAksorLYZkUfswwp74Bq9PPjX";
const RPC_URL: &str = "https://api.devnet.solana.com";

#[derive(Serialize, Deserialize)]
struct Image {
    url: String,
    title: String
}

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let operation = args.next().unwrap();

    match operation.as_str() {
        "add" => {
            let keypair = args.next().unwrap();
            let url = args.next().unwrap();
            let title = args.next().unwrap();

            let image = Image {
                url,
                title
            };

            add_image(keypair, image);
        },
        "close" => {
            let keypair = args.next().unwrap();
            let pda = args.next().unwrap();

            close_account(keypair, pda);
        },
        "read-pda" => {
            let pda = args.next().unwrap();

            read_pda(pda);
        },
        _ => ()
    }
}

fn add_image(keypair: String, image: Image) {
    let program_id = Pubkey::try_from(PROGRAM_ID).unwrap();
    let signer = keypair::read_keypair_file(keypair).unwrap();

    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[signer.encodable_pubkey().as_ref(), image.title.as_bytes()],
        &program_id
    );

    let instruction = Instruction::new_with_bincode(
        program_id,
        &image,
        vec![
            AccountMeta::new(signer.encodable_pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new_readonly(system_program::id(), false)
        ]
    );

    let client = RpcClient::new(RPC_URL);
    let recent_blockhash = client.get_latest_blockhash().unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.encodable_pubkey()),
        &[signer],
        recent_blockhash
    );

    let signature = client.send_and_confirm_transaction(&transaction).unwrap();

    println!("Image Added");
    println!("Signature: {signature}");
}

fn close_account(keypair: String, pda: String) {
    let program_id = Pubkey::try_from(PROGRAM_ID).unwrap();
    let signer = keypair::read_keypair_file(keypair).unwrap();
    let pda = Pubkey::try_from(pda.as_str()).unwrap();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &[],
        vec![
            AccountMeta::new(signer.encodable_pubkey(), true),
            AccountMeta::new(pda, false),
            AccountMeta::new_readonly(system_program::id(), false)
        ]
    );

    let client = RpcClient::new(RPC_URL);
    let recent_blockhash = client.get_latest_blockhash().unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&signer.encodable_pubkey()),
        &[signer],
        recent_blockhash
    );

    let signature = client.send_and_confirm_transaction(&transaction).unwrap();

    println!("PDA Closed");
    println!("Signature: {signature}");
}

fn read_pda(pda: String) {
    let pubkey = Pubkey::try_from(pda.as_str()).unwrap();
    let client = RpcClient::new(RPC_URL);

    let data = client.get_account_data(&pubkey).unwrap();
    let image: Image = bincode::deserialize(&data).unwrap();

    println!("Image Title: {}", image.title);
    println!("Image URL: {}", image.url);
}
