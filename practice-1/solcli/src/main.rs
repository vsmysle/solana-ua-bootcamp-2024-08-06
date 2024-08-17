use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate,
    ShowPublicKey,
    ShowBalance,
    Airdrop { 
        amount: f64 
    },
}

fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new(rpc_url);

    match &cli.command {
        Commands::Generate => generate_keypair()?,
        Commands::ShowPublicKey => show_public_key()?,
        Commands::ShowBalance => show_balance(&client)?,
        Commands::Airdrop { amount } => request_airdrop(&client, *amount)?,
    }

    Ok(())
}

fn generate_keypair() -> Result<()> {
    let keypair = Keypair::new();
    println!("New keypair generated:");
    println!("Public key: {}", keypair.pubkey());
    println!("Private key: {}", bs58::encode(keypair.to_bytes()).into_string());
    println!("Remember to update your .env file with the new private key!");
    Ok(())
}

fn show_public_key() -> Result<()> {
    let keypair = get_keypair_from_env()?;
    println!("Public key: {}", keypair.pubkey());
    Ok(())
}

fn show_balance(client: &RpcClient) -> Result<()> {
    let keypair = get_keypair_from_env()?;
    let balance = client.get_balance(&keypair.pubkey())?;
    println!("Balance: {} SOL", lamports_to_sol(balance));
    Ok(())
}

fn request_airdrop(client: &RpcClient, amount: f64) -> Result<()> {
    let keypair = get_keypair_from_env()?;
    let amount_lamports = sol_to_lamports(amount);
    let signature = client.request_airdrop(&keypair.pubkey(), amount_lamports)?;
    client.confirm_transaction(&signature)?;
    println!("Airdrop of {} SOL successful", amount);
    Ok(())
}

fn get_keypair_from_env() -> Result<Keypair> {
    let private_key = std::env::var("PRIVATE_KEY").map_err(|_| anyhow!("PRIVATE_KEY not found in .env"))?;
    let private_key_bytes = bs58::decode(private_key).into_vec()?;
    Ok(Keypair::from_bytes(&private_key_bytes)?)
}

fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1_000_000_000.0
}

fn sol_to_lamports(sol: f64) -> u64 {
    (sol * 1_000_000_000.0) as u64
}
